use crate::bindings;
use crate::error::attach_handle_to_error;
use crate::error::generic_error;
use crate::error::ErrWithV8Handle;
use crate::error::JsError;
use crate::extensions::Extension;
use crate::extensions::OpDecl;
use crate::extensions::OpEventLoopFn;
use crate::extensions::OpMiddlewareFn;
use crate::inspector::JsRuntimeInspector;
use crate::module_specifier::ModuleSpecifier;
use crate::modules::ModuleId;
use crate::modules::ModuleLoadId;
use crate::modules::ModuleLoader;
use crate::modules::ModuleMap;
use crate::modules::NoopModuleLoader;
use crate::ops::*;
use crate::ops_builtin::op_void_async;
use crate::ops_builtin::op_void_sync;
use anyhow::Error;
use futures::channel::oneshot;
use futures::future::poll_fn;
use futures::future::Future;
use futures::future::FutureExt;
use futures::stream::FuturesUnordered;
use futures::stream::StreamExt;
use futures::task::AtomicWaker;
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ffi::c_void;
use std::mem::forget;
use std::option::Option;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::Once;
use std::task::Context;
use std::task::Poll;
type PendingOpFuture = OpCall<(PromiseId, OpId, OpResult)>;

pub enum Snapshot {
    Static(&'static [u8]),
    JustCreated(v8::StartupData),
    Boxed(Box<[u8]>),
}

pub type JsErrorCreateFn = dyn Fn(JsError) -> Error;

pub type GetErrorClassFn = &'static dyn for<'e> Fn(&'e Error) -> &'static str;

/// Objects that need to live as long as the isolate
#[derive(Default)]
struct IsolateAllocations {
    near_heap_limit_callback_data: Option<(Box<RefCell<dyn Any>>, v8::NearHeapLimitCallback)>,
}

pub struct JsRuntime {
    // This is an Option<OwnedIsolate> instead of just OwnedIsolate to workaround
    // a safety issue with SnapshotCreator. See JsRuntime::drop.
    v8_isolate: Option<v8::OwnedIsolate>,
    // This is an Option<Box<JsRuntimeInspector> instead of just Box<JsRuntimeInspector>
    // to workaround a safety issue. See JsRuntime::drop.
    inspector: Option<Box<JsRuntimeInspector>>,
    snapshot_creator: Option<v8::SnapshotCreator>,
    has_snapshotted: bool,
    allocations: IsolateAllocations,
    extensions: Vec<Extension>,
    event_loop_middlewares: Vec<Box<OpEventLoopFn>>,
}

struct DynImportModEvaluate {
    load_id: ModuleLoadId,
    module_id: ModuleId,
    promise: v8::Global<v8::Promise>,
    module: v8::Global<v8::Module>,
}

struct ModEvaluate {
    promise: v8::Global<v8::Promise>,
    sender: oneshot::Sender<Result<(), Error>>,
}

pub struct CrossIsolateStore<T>(Arc<Mutex<CrossIsolateStoreInner<T>>>);

struct CrossIsolateStoreInner<T> {
    map: HashMap<u32, T>,
    last_id: u32,
}

impl<T> CrossIsolateStore<T> {
    pub(crate) fn insert(&self, value: T) -> u32 {
        let mut store = self.0.lock().unwrap();
        let last_id = store.last_id;
        store.map.insert(last_id, value);
        store.last_id += 1;
        last_id
    }

    pub(crate) fn take(&self, id: u32) -> Option<T> {
        let mut store = self.0.lock().unwrap();
        store.map.remove(&id)
    }
}

impl<T> Default for CrossIsolateStore<T> {
    fn default() -> Self {
        CrossIsolateStore(Arc::new(Mutex::new(CrossIsolateStoreInner {
            map: Default::default(),
            last_id: 0,
        })))
    }
}

impl<T> Clone for CrossIsolateStore<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

pub type SharedArrayBufferStore = CrossIsolateStore<v8::SharedRef<v8::BackingStore>>;

pub type CompiledWasmModuleStore = CrossIsolateStore<v8::CompiledWasmModule>;

/// Internal state for JsRuntime which is stored in one of v8::Isolate's
/// embedder slots.
pub(crate) struct JsRuntimeState {
    pub global_context: Option<v8::Global<v8::Context>>,
    pub(crate) js_recv_cb: Option<v8::Global<v8::Function>>,
    pub(crate) js_macrotask_cbs: Vec<v8::Global<v8::Function>>,
    pub(crate) js_nexttick_cbs: Vec<v8::Global<v8::Function>>,
    pub(crate) js_promise_reject_cb: Option<v8::Global<v8::Function>>,
    pub(crate) js_uncaught_exception_cb: Option<v8::Global<v8::Function>>,
    pub(crate) has_tick_scheduled: bool,
    pub(crate) js_wasm_streaming_cb: Option<v8::Global<v8::Function>>,
    pub(crate) pending_promise_exceptions: HashMap<v8::Global<v8::Promise>, v8::Global<v8::Value>>,
    pending_dyn_mod_evaluate: Vec<DynImportModEvaluate>,
    pending_mod_evaluate: Option<ModEvaluate>,
    /// A counter used to delay our dynamic import deadlock detection by one spin
    /// of the event loop.
    dyn_module_evaluate_idle_counter: u32,
    pub(crate) js_error_create_fn: Rc<JsErrorCreateFn>,
    pub(crate) pending_ops: FuturesUnordered<PendingOpFuture>,
    pub(crate) unrefed_ops: HashSet<i32>,
    pub(crate) have_unpolled_ops: bool,
    pub(crate) op_state: Rc<RefCell<OpState>>,
    #[allow(dead_code)]
    // We don't explicitly re-read this prop but need the slice to live alongside the isolate
    pub(crate) op_ctxs: Box<[OpCtx]>,
    pub(crate) shared_array_buffer_store: Option<SharedArrayBufferStore>,
    pub(crate) compiled_wasm_module_store: Option<CompiledWasmModuleStore>,
    /// The error that was passed to an explicit `Cassie.core.terminate` call.
    /// It will be retrieved by `exception_to_err_result` and used as an error
    /// instead of any other exceptions.
    pub(crate) explicit_terminate_exception: Option<v8::Global<v8::Value>>,
    waker: AtomicWaker,
}

impl Drop for JsRuntime {
    fn drop(&mut self) {
        // The Isolate object must outlive the Inspector object, but this is
        // currently not enforced by the type system.
        self.inspector.take();

        if let Some(creator) = self.snapshot_creator.take() {
            // TODO(ry): in rusty_v8, `SnapShotCreator::get_owned_isolate()` returns
            // a `struct OwnedIsolate` which is not actually owned, hence the need
            // here to leak the `OwnedIsolate` in order to avoid a double free and
            // the segfault that it causes.
            let v8_isolate = self.v8_isolate.take().unwrap();
            forget(v8_isolate);

            // TODO(ry) V8 has a strange assert which prevents a SnapshotCreator from
            // being deallocated if it hasn't created a snapshot yet.
            // https://github.com/v8/v8/blob/73212783fbd534fac76cc4b66aac899c13f71fc8/src/api.cc#L603
            // If that assert is removed, this if guard could be removed.
            // WARNING: There may be false positive LSAN errors here.
            if self.has_snapshotted {
                drop(creator);
            }
        }
    }
}
///初始化 v8_platform 并设置 v8_platform 的线程池大小
fn v8_init(v8_platform: Option<v8::SharedRef<v8::Platform>>) {
    // Include 10MB ICU data file.
    #[repr(C, align(16))]
    struct IcuData([u8; 10284336]);
    static ICU_DATA: IcuData = IcuData(*include_bytes!("icudtl.dat"));
    v8::icu::set_common_data_70(&ICU_DATA.0).unwrap();

    let v8_platform =
        v8_platform.unwrap_or_else(|| v8::new_default_platform(0, false).make_shared());
    v8::V8::initialize_platform(v8_platform);
    v8::V8::initialize();

    let flags = concat!(
        " --experimental-wasm-threads",
        " --wasm-test-streaming",
        " --harmony-import-assertions",
        " --no-validate-asm",
    );
    v8::V8::set_flags_from_string(flags);
}

#[derive(Default)]
pub struct RuntimeOptions {
    /// Allows a callback to be set whenever a V8 exception is made. This allows
    /// the caller to wrap the JsError into an error. By default this callback
    /// is set to `JsError::create()`.
    pub js_error_create_fn: Option<Rc<JsErrorCreateFn>>,

    /// Allows to map error type to a string "class" used to represent
    /// error in JavaScript.
    pub get_error_class_fn: Option<GetErrorClassFn>,

    /// Implementation of `ModuleLoader` which will be
    /// called when V8 requests to load ES modules.
    ///
    /// If not provided runtime will error if code being
    /// executed tries to load modules.
    pub module_loader: Option<Rc<dyn ModuleLoader>>,

    /// JsRuntime extensions, not to be confused with ES modules
    /// these are sets of ops and other JS code to be initialized.
    pub extensions: Vec<Extension>,

    /// V8 snapshot that should be loaded on startup.
    ///
    /// Currently can't be used with `will_snapshot`.
    pub startup_snapshot: Option<Snapshot>,

    /// Prepare runtime to take snapshot of loaded code.
    ///
    /// Currently can't be used with `startup_snapshot`.
    pub will_snapshot: bool,

    /// Isolate creation parameters.
    pub create_params: Option<v8::CreateParams>,

    /// V8 platform instance to use. Used when Cassie initializes V8
    /// (which it only does once), otherwise it's silenty dropped.
    pub v8_platform: Option<v8::SharedRef<v8::Platform>>,

    /// The store to use for transferring SharedArrayBuffers between isolates.
    /// If multiple isolates should have the possibility of sharing
    /// SharedArrayBuffers, they should use the same [SharedArrayBufferStore]. If
    /// no [SharedArrayBufferStore] is specified, SharedArrayBuffer can not be
    /// serialized.
    pub shared_array_buffer_store: Option<SharedArrayBufferStore>,

    /// The store to use for transferring `WebAssembly.Module` objects between
    /// isolates.
    /// If multiple isolates should have the possibility of sharing
    /// `WebAssembly.Module` objects, they should use the same
    /// [CompiledWasmModuleStore]. If no [CompiledWasmModuleStore] is specified,
    /// `WebAssembly.Module` objects cannot be serialized.
    pub compiled_wasm_module_store: Option<CompiledWasmModuleStore>,
}

impl JsRuntime {

    /// JsRuntime 构造函数，通过 `options` 配置
    pub fn new(mut options: RuntimeOptions) -> Self {
        let v8_platform = options.v8_platform.take();
        //初始化 v8_platform 只能初始化一次
        static CASSIE_INIT: Once = Once::new();
        CASSIE_INIT.call_once(move || v8_init(v8_platform));
        ///是否有脚本快照
        let has_startup_snapshot = options.startup_snapshot.is_some();

        let js_error_create_fn = options
            .js_error_create_fn
            .unwrap_or_else(|| Rc::new(JsError::create));

        // Add builtins extension
        ///添加内置拓展 自定义方法
        options
            .extensions
            .insert(0, crate::ops_builtin::init_builtins());

        let ops = Self::collect_ops(&mut options.extensions);
        let mut op_state = OpState::new(ops.len());

        if let Some(get_error_class_fn) = options.get_error_class_fn {
            op_state.get_error_class_fn = get_error_class_fn;
        }

        let op_state = Rc::new(RefCell::new(op_state));
        let op_ctxs = ops
            .into_iter()
            .enumerate()
            .map(|(id, decl)| OpCtx {
                id,
                state: op_state.clone(),
                decl,
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();

        let global_context;
        //快照读取
        let (mut isolate, maybe_snapshot_creator) = if options.will_snapshot {
            // TODO(ry) Support loading snapshots before snapshotting.
            assert!(options.startup_snapshot.is_none());
            let mut creator = v8::SnapshotCreator::new(Some(&bindings::EXTERNAL_REFERENCES));
            let isolate = unsafe { creator.get_owned_isolate() };
            let mut isolate = JsRuntime::setup_isolate(isolate);
            {
                let scope = &mut v8::HandleScope::new(&mut isolate);
                let context = bindings::initialize_context(scope, &op_ctxs, false);
                global_context = v8::Global::new(scope, context);
                creator.set_default_context(context);
            }
            (isolate, Some(creator))
        } else {
            //如果不是快照，则创建一个新的 isolate
            let mut params = options
                .create_params
                .take()
                .unwrap_or_else(v8::Isolate::create_params)
                .external_references(&**bindings::EXTERNAL_REFERENCES);
            let snapshot_loaded = if let Some(snapshot) = options.startup_snapshot {
                params = match snapshot {
                    Snapshot::Static(data) => params.snapshot_blob(data),
                    Snapshot::JustCreated(data) => params.snapshot_blob(data),
                    Snapshot::Boxed(data) => params.snapshot_blob(data),
                };
                true
            } else {
                false
            };

            let isolate = v8::Isolate::new(params);
            let mut isolate = JsRuntime::setup_isolate(isolate);
            {
                let scope = &mut v8::HandleScope::new(&mut isolate);
                let context = bindings::initialize_context(scope, &op_ctxs, snapshot_loaded);

                global_context = v8::Global::new(scope, context);
            }
            (isolate, None)
        };

        let inspector = JsRuntimeInspector::new(&mut isolate, global_context.clone());

        let loader = options
            .module_loader
            .unwrap_or_else(|| Rc::new(NoopModuleLoader));

        isolate.set_slot(Rc::new(RefCell::new(JsRuntimeState {
            global_context: Some(global_context),
            pending_promise_exceptions: HashMap::new(),
            pending_dyn_mod_evaluate: vec![],
            pending_mod_evaluate: None,
            dyn_module_evaluate_idle_counter: 0,
            js_recv_cb: None,
            js_macrotask_cbs: vec![],
            js_nexttick_cbs: vec![],
            js_promise_reject_cb: None,
            js_uncaught_exception_cb: None,
            has_tick_scheduled: false,
            js_wasm_streaming_cb: None,
            js_error_create_fn,
            pending_ops: FuturesUnordered::new(),
            unrefed_ops: HashSet::new(),
            shared_array_buffer_store: options.shared_array_buffer_store,
            compiled_wasm_module_store: options.compiled_wasm_module_store,
            op_state: op_state.clone(),
            op_ctxs,
            have_unpolled_ops: false,
            explicit_terminate_exception: None,
            waker: AtomicWaker::new(),
        })));

        let module_map = ModuleMap::new(loader, op_state);
        isolate.set_slot(Rc::new(RefCell::new(module_map)));

        let mut js_runtime = Self {
            v8_isolate: Some(isolate),
            inspector: Some(inspector),
            snapshot_creator: maybe_snapshot_creator,
            has_snapshotted: false,
            allocations: IsolateAllocations::default(),
            event_loop_middlewares: Vec::with_capacity(options.extensions.len()),
            extensions: options.extensions,
        };

        // TODO(@AaronO): diff extensions inited in snapshot and those provided
        // for now we assume that snapshot and extensions always match
        if !has_startup_snapshot {
            js_runtime.init_extension_js().unwrap();
        }
        // Init extension ops
        js_runtime.init_extension_ops().unwrap();
        // Init callbacks (opresolve)
        js_runtime.init_cbs();

        js_runtime
    }

    pub fn global_context(&mut self) -> v8::Global<v8::Context> {
        let state = Self::state(self.v8_isolate());
        let state = state.borrow();
        state.global_context.clone().unwrap()
    }

    pub fn v8_isolate(&mut self) -> &mut v8::OwnedIsolate {
        self.v8_isolate.as_mut().unwrap()
    }

    pub fn inspector(&mut self) -> &mut Box<JsRuntimeInspector> {
        self.inspector.as_mut().unwrap()
    }

    pub fn handle_scope(&mut self) -> v8::HandleScope {
        let context = self.global_context();
        v8::HandleScope::with_context(self.v8_isolate(), context)
    }

    fn setup_isolate(mut isolate: v8::OwnedIsolate) -> v8::OwnedIsolate {
        isolate.set_capture_stack_trace_for_uncaught_exceptions(true, 10);
        isolate.set_promise_reject_callback(bindings::promise_reject_callback);
        isolate.set_host_initialize_import_meta_object_callback(
            bindings::host_initialize_import_meta_object_callback,
        );
        isolate.set_host_import_module_dynamically_callback(
            bindings::host_import_module_dynamically_callback,
        );
        isolate
    }

    pub(crate) fn state(isolate: &v8::Isolate) -> Rc<RefCell<JsRuntimeState>> {
        let s = isolate.get_slot::<Rc<RefCell<JsRuntimeState>>>().unwrap();
        s.clone()
    }

    pub(crate) fn module_map(isolate: &v8::Isolate) -> Rc<RefCell<ModuleMap>> {
        let module_map = isolate.get_slot::<Rc<RefCell<ModuleMap>>>().unwrap();
        module_map.clone()
    }

    /// Initializes JS of provided Extensions
    fn init_extension_js(&mut self) -> Result<(), Error> {
        // Take extensions to avoid double-borrow
        let mut extensions: Vec<Extension> = std::mem::take(&mut self.extensions);
        for m in extensions.iter_mut() {
            let js_files = m.init_js();
            for (filename, source) in js_files {
                let source = source()?;
                // TODO(@AaronO): use JsRuntime::execute_static() here to move src off heap
                self.execute_script(filename, &source)?;
            }
        }
        // Restore extensions
        self.extensions = extensions;

        Ok(())
    }

    /// Collects ops from extensions & applies middleware
    fn collect_ops(extensions: &mut [Extension]) -> Vec<OpDecl> {
        // Middleware
        let middleware: Vec<Box<OpMiddlewareFn>> = extensions
            .iter_mut()
            .filter_map(|e| e.init_middleware())
            .collect();

        // macroware wraps an opfn in all the middleware
        let macroware = move |d| middleware.iter().fold(d, |d, m| m(d));

        // Flatten ops, apply middlware & override disabled ops
        extensions
            .iter_mut()
            .filter_map(|e| e.init_ops())
            .flatten()
            .map(|d| OpDecl {
                name: d.name,
                ..macroware(d)
            })
            .map(|op| match op.enabled {
                true => op,
                false => OpDecl {
                    v8_fn_ptr: match op.is_async {
                        true => op_void_async::v8_fn_ptr(),
                        false => op_void_sync::v8_fn_ptr(),
                    },
                    ..op
                },
            })
            .collect()
    }

    /// Initializes ops of provided Extensions
    fn init_extension_ops(&mut self) -> Result<(), Error> {
        let op_state = self.op_state();
        // Take extensions to avoid double-borrow
        let mut extensions: Vec<Extension> = std::mem::take(&mut self.extensions);

        // Setup state
        for e in extensions.iter_mut() {
            // ops are already registered during in bindings::initialize_context();
            e.init_state(&mut op_state.borrow_mut())?;

            // Setup event-loop middleware
            if let Some(middleware) = e.init_event_loop_middleware() {
                self.event_loop_middlewares.push(middleware);
            }
        }

        // Restore extensions
        self.extensions = extensions;

        Ok(())
    }

    /// Grab a Global handle to a v8 value returned by the expression
    pub(crate) fn grab<'s, T>(
        scope: &mut v8::HandleScope<'s>,
        root: v8::Local<'s, v8::Value>,
        path: &str,
    ) -> Option<v8::Local<'s, T>>
    where
        v8::Local<'s, T>: TryFrom<v8::Local<'s, v8::Value>, Error = v8::DataError>,
    {
        path.split('.')
            .fold(Some(root), |p, k| {
                let p = v8::Local::<v8::Object>::try_from(p?).ok()?;
                let k = v8::String::new(scope, k)?;
                p.get(scope, k.into())
            })?
            .try_into()
            .ok()
    }

    pub(crate) fn grab_global<'s, T>(
        scope: &mut v8::HandleScope<'s>,
        path: &str,
    ) -> Option<v8::Local<'s, T>>
    where
        v8::Local<'s, T>: TryFrom<v8::Local<'s, v8::Value>, Error = v8::DataError>,
    {
        let context = scope.get_current_context();
        let global = context.global(scope);
        Self::grab(scope, global.into(), path)
    }

    pub(crate) fn ensure_objs<'s>(
        scope: &mut v8::HandleScope<'s>,
        root: v8::Local<'s, v8::Object>,
        path: &str,
    ) -> Option<v8::Local<'s, v8::Object>> {
        path.split('.').fold(Some(root), |p, k| {
            let k = v8::String::new(scope, k)?.into();
            match p?.get(scope, k) {
                Some(v) if !v.is_null_or_undefined() => v.try_into().ok(),
                _ => {
                    let o = v8::Object::new(scope);
                    p?.set(scope, k, o.into());
                    Some(o)
                }
            }
        })
    }

    /// Grabs a reference to core.js' opresolve & syncOpsCache()
    fn init_cbs(&mut self) {
        let scope = &mut self.handle_scope();
        let recv_cb = Self::grab_global::<v8::Function>(scope, "Cassie.core.opresolve").unwrap();
        let recv_cb = v8::Global::new(scope, recv_cb);
        // Put global handles in state
        let state_rc = JsRuntime::state(scope);
        let mut state = state_rc.borrow_mut();
        state.js_recv_cb.replace(recv_cb);
    }

    /// Returns the runtime's op state, which can be used to maintain ops
    /// and access resources between op calls.
    pub fn op_state(&mut self) -> Rc<RefCell<OpState>> {
        let state_rc = Self::state(self.v8_isolate());
        let state = state_rc.borrow();
        state.op_state.clone()
    }

    /// Executes traditional JavaScript code (traditional = not ES modules).
    ///
    /// The execution takes place on the current global context, so it is possible
    /// to maintain local JS state and invoke this method multiple times.
    ///
    /// `name` can be a filepath or any other string, eg.
    ///
    ///   - "/some/file/path.js"
    ///   - "<anon>"
    ///   - "[native code]"
    ///
    /// The same `name` value can be used for multiple executions.
    ///
    /// `Error` can be downcast to a type that exposes additional information
    /// about the V8 exception. By default this type is `JsError`, however it may
    /// be a different type if `RuntimeOptions::js_error_create_fn` has been set.
    pub fn execute_script(
        &mut self,
        name: &str,
        source_code: &str,
    ) -> Result<v8::Global<v8::Value>, Error> {
        let scope = &mut self.handle_scope();

        let source = v8::String::new(scope, source_code).unwrap();
        let name = v8::String::new(scope, name).unwrap();
        let origin = bindings::script_origin(scope, name);

        let tc_scope = &mut v8::TryCatch::new(scope);

        let script = match v8::Script::compile(tc_scope, source, Some(&origin)) {
            Some(script) => script,
            None => {
                let exception = tc_scope.exception().unwrap();
                return exception_to_err_result(tc_scope, exception, false);
            }
        };

        match script.run(tc_scope) {
            Some(value) => {
                let value_handle = v8::Global::new(tc_scope, value);
                Ok(value_handle)
            }
            None => {
                assert!(tc_scope.has_caught());
                let exception = tc_scope.exception().unwrap();
                exception_to_err_result(tc_scope, exception, false)
            }
        }
    }

    /// Takes a snapshot. The isolate should have been created with will_snapshot
    /// set to true.
    ///
    /// `Error` can be downcast to a type that exposes additional information
    /// about the V8 exception. By default this type is `JsError`, however it may
    /// be a different type if `RuntimeOptions::js_error_create_fn` has been set.
    pub fn snapshot(&mut self) -> v8::StartupData {
        assert!(self.snapshot_creator.is_some());

        // Nuke Cassie.core.ops.* to avoid ExternalReference snapshotting issues
        // TODO(@AaronO): make ops stable across snapshots
        {
            let scope = &mut self.handle_scope();
            let o = Self::grab_global::<v8::Object>(scope, "Cassie.core.ops").unwrap();
            let names = o.get_own_property_names(scope).unwrap();
            for i in 0..names.length() {
                let key = names.get_index(scope, i).unwrap();
                o.delete(scope, key);
            }
        }

        let state = Self::state(self.v8_isolate());

        state.borrow_mut().global_context.take();

        self.inspector.take();

        // Overwrite existing ModuleMap to drop v8::Global handles
        self.v8_isolate()
            .set_slot(Rc::new(RefCell::new(ModuleMap::new(
                Rc::new(NoopModuleLoader),
                state.borrow().op_state.clone(),
            ))));
        // Drop other v8::Global handles before snapshotting
        std::mem::take(&mut state.borrow_mut().js_recv_cb);

        let snapshot_creator = self.snapshot_creator.as_mut().unwrap();
        let snapshot = snapshot_creator
            .create_blob(v8::FunctionCodeHandling::Keep)
            .unwrap();
        self.has_snapshotted = true;

        snapshot
    }

    /// Returns the namespace object of a module.
    ///
    /// This is only available after module evaluation has completed.
    /// This function panics if module has not been instantiated.
    pub fn get_module_namespace(
        &mut self,
        module_id: ModuleId,
    ) -> Result<v8::Global<v8::Object>, Error> {
        let module_map_rc = Self::module_map(self.v8_isolate());

        let module_handle = module_map_rc
            .borrow()
            .get_handle(module_id)
            .expect("ModuleInfo not found");

        let scope = &mut self.handle_scope();

        let module = module_handle.open(scope);

        if module.get_status() == v8::ModuleStatus::Errored {
            let exception = module.get_exception();
            let err = exception_to_err_result(scope, exception, false)
                .map_err(|err| attach_handle_to_error(scope, err, exception));
            return err;
        }

        assert!(matches!(
            module.get_status(),
            v8::ModuleStatus::Instantiated | v8::ModuleStatus::Evaluated
        ));

        let module_namespace: v8::Local<v8::Object> =
            v8::Local::try_from(module.get_module_namespace())
                .map_err(|err: v8::DataError| generic_error(err.to_string()))?;

        Ok(v8::Global::new(scope, module_namespace))
    }

    /// Registers a callback on the isolate when the memory limits are approached.
    /// Use this to prevent V8 from crashing the process when reaching the limit.
    ///
    /// Calls the closure with the current heap limit and the initial heap limit.
    /// The return value of the closure is set as the new limit.
    pub fn add_near_heap_limit_callback<C>(&mut self, cb: C)
    where
        C: FnMut(usize, usize) -> usize + 'static,
    {
        let boxed_cb = Box::new(RefCell::new(cb));
        let data = boxed_cb.as_ptr() as *mut c_void;

        let prev = self
            .allocations
            .near_heap_limit_callback_data
            .replace((boxed_cb, near_heap_limit_callback::<C>));
        if let Some((_, prev_cb)) = prev {
            self.v8_isolate()
                .remove_near_heap_limit_callback(prev_cb, 0);
        }

        self.v8_isolate()
            .add_near_heap_limit_callback(near_heap_limit_callback::<C>, data);
    }

    pub fn remove_near_heap_limit_callback(&mut self, heap_limit: usize) {
        if let Some((_, cb)) = self.allocations.near_heap_limit_callback_data.take() {
            self.v8_isolate()
                .remove_near_heap_limit_callback(cb, heap_limit);
        }
    }

    fn pump_v8_message_loop(&mut self) {
        let scope = &mut self.handle_scope();
        while v8::Platform::pump_message_loop(
            &v8::V8::get_current_platform(),
            scope,
            false, // don't block if there are no tasks
        ) {
            // do nothing
        }

        scope.perform_microtask_checkpoint();
    }

    pub fn poll_value(
        &mut self,
        global: &v8::Global<v8::Value>,
        cx: &mut Context,
    ) -> Poll<Result<v8::Global<v8::Value>, Error>> {
        let state = self.poll_event_loop(cx, false);

        let mut scope = self.handle_scope();
        let local = v8::Local::<v8::Value>::new(&mut scope, global);

        if let Ok(promise) = v8::Local::<v8::Promise>::try_from(local) {
            match promise.state() {
                v8::PromiseState::Pending => match state {
                    Poll::Ready(Ok(_)) => {
                        let msg = "Promise resolution is still pending but the event loop has already resolved.";
                        Poll::Ready(Err(generic_error(msg)))
                    }
                    Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
                    Poll::Pending => Poll::Pending,
                },
                v8::PromiseState::Fulfilled => {
                    let value = promise.result(&mut scope);
                    let value_handle = v8::Global::new(&mut scope, value);
                    Poll::Ready(Ok(value_handle))
                }
                v8::PromiseState::Rejected => {
                    let exception = promise.result(&mut scope);
                    Poll::Ready(exception_to_err_result(&mut scope, exception, false))
                }
            }
        } else {
            let value_handle = v8::Global::new(&mut scope, local);
            Poll::Ready(Ok(value_handle))
        }
    }

    /// Waits for the given value to resolve while polling the event loop.
    ///
    /// This future resolves when either the value is resolved or the event loop runs to
    /// completion.
    pub async fn resolve_value(
        &mut self,
        global: v8::Global<v8::Value>,
    ) -> Result<v8::Global<v8::Value>, Error> {
        poll_fn(|cx| self.poll_value(&global, cx)).await
    }

    /// Runs event loop to completion
    ///
    /// This future resolves when:
    ///  - there are no more pending dynamic imports
    ///  - there are no more pending ops
    ///  - there are no more active inspector sessions (only if `wait_for_inspector` is set to true)
    pub async fn run_event_loop(&mut self, wait_for_inspector: bool) -> Result<(), Error> {
        poll_fn(|cx| self.poll_event_loop(cx, wait_for_inspector)).await
    }

    /// Runs a single tick of event loop
    ///
    /// If `wait_for_inspector` is set to true event loop
    /// will return `Poll::Pending` if there are active inspector sessions.
    pub fn poll_event_loop(
        &mut self,
        cx: &mut Context,
        wait_for_inspector: bool,
    ) -> Poll<Result<(), Error>> {
        // We always poll the inspector first
        let _ = self.inspector().poll_unpin(cx);

        let state_rc = Self::state(self.v8_isolate());
        let module_map_rc = Self::module_map(self.v8_isolate());
        {
            let state = state_rc.borrow();
            state.waker.register(cx.waker());
        }

        self.pump_v8_message_loop();

        // Ops
        {
            self.resolve_async_ops(cx)?;
            self.drain_nexttick()?;
            self.drain_macrotasks()?;
            self.check_promise_exceptions()?;
        }

        // Dynamic module loading - ie. modules loaded using "import()"
        {
            let poll_imports = self.prepare_dyn_imports(cx)?;
            assert!(poll_imports.is_ready());

            let poll_imports = self.poll_dyn_imports(cx)?;
            assert!(poll_imports.is_ready());

            self.evaluate_dyn_imports();

            self.check_promise_exceptions()?;
        }

        // Event loop middlewares
        let mut maybe_scheduling = false;
        {
            let state = state_rc.borrow();
            let op_state = state.op_state.clone();
            for f in &self.event_loop_middlewares {
                if f(&mut op_state.borrow_mut(), cx) {
                    maybe_scheduling = true;
                }
            }
        }

        // Top level module
        self.evaluate_pending_module();

        let mut state = state_rc.borrow_mut();
        let module_map = module_map_rc.borrow();

        let has_pending_refed_ops = state.pending_ops.len() > state.unrefed_ops.len();
        let has_pending_dyn_imports = module_map.has_pending_dynamic_imports();
        let has_pending_dyn_module_evaluation = !state.pending_dyn_mod_evaluate.is_empty();
        let has_pending_module_evaluation = state.pending_mod_evaluate.is_some();
        let has_pending_background_tasks = self.v8_isolate().has_pending_background_tasks();
        let has_tick_scheduled = state.has_tick_scheduled;
        let inspector_has_active_sessions = self
            .inspector
            .as_ref()
            .map(|i| i.has_active_sessions())
            .unwrap_or(false);

        if !has_pending_refed_ops
            && !has_pending_dyn_imports
            && !has_pending_dyn_module_evaluation
            && !has_pending_module_evaluation
            && !has_pending_background_tasks
            && !has_tick_scheduled
            && !maybe_scheduling
        {
            if wait_for_inspector && inspector_has_active_sessions {
                return Poll::Pending;
            }

            return Poll::Ready(Ok(()));
        }

        // Check if more async ops have been dispatched
        // during this turn of event loop.
        // If there are any pending background tasks, we also wake the runtime to
        // make sure we don't miss them.
        // TODO(andreubotella) The event loop will spin as long as there are pending
        // background tasks. We should look into having V8 notify us when a
        // background task is done.
        if state.have_unpolled_ops
            || has_pending_background_tasks
            || has_tick_scheduled
            || maybe_scheduling
        {
            state.waker.wake();
        }

        if has_pending_module_evaluation {
            if has_pending_refed_ops
                || has_pending_dyn_imports
                || has_pending_dyn_module_evaluation
                || has_pending_background_tasks
                || has_tick_scheduled
                || maybe_scheduling
            {
                // pass, will be polled again
            } else {
                let msg = "Module evaluation is still pending but there are no pending ops or dynamic imports. This situation is often caused by unresolved promises.";
                return Poll::Ready(Err(generic_error(msg)));
            }
        }

        if has_pending_dyn_module_evaluation {
            if has_pending_refed_ops
                || has_pending_dyn_imports
                || has_pending_background_tasks
                || has_tick_scheduled
            {
                // pass, will be polled again
            } else if state.dyn_module_evaluate_idle_counter >= 1 {
                let mut msg = "Dynamically imported module evaluation is still pending but there are no pending ops. This situation is often caused by unresolved promises.
Pending dynamic modules:\n".to_string();
                for pending_evaluate in &state.pending_dyn_mod_evaluate {
                    let module_info = module_map
                        .get_info_by_id(&pending_evaluate.module_id)
                        .unwrap();
                    msg.push_str(&format!("- {}", module_info.name.as_str()));
                }
                return Poll::Ready(Err(generic_error(msg)));
            } else {
                // Delay the above error by one spin of the event loop. A dynamic import
                // evaluation may complete during this, in which case the counter will
                // reset.
                state.dyn_module_evaluate_idle_counter += 1;
                state.waker.wake();
            }
        }

        Poll::Pending
    }
}

extern "C" fn near_heap_limit_callback<F>(
    data: *mut c_void,
    current_heap_limit: usize,
    initial_heap_limit: usize,
) -> usize
where
    F: FnMut(usize, usize) -> usize,
{
    let callback = unsafe { &mut *(data as *mut F) };
    callback(current_heap_limit, initial_heap_limit)
}

impl JsRuntimeState {
    /// Called by `bindings::host_import_module_dynamically_callback`
    /// after initiating new dynamic import load.
    pub fn notify_new_dynamic_import(&mut self) {
        // Notify event loop to poll again soon.
        self.waker.wake();
    }
}

pub(crate) fn exception_to_err_result<'s, T>(
    scope: &mut v8::HandleScope<'s>,
    exception: v8::Local<v8::Value>,
    in_promise: bool,
) -> Result<T, Error> {
    let state_rc = JsRuntime::state(scope);
    let mut state = state_rc.borrow_mut();

    let is_terminating_exception = scope.is_execution_terminating();
    let mut exception = exception;

    if is_terminating_exception {
        // TerminateExecution was called. Cancel isolate termination so that the
        // exception can be created..
        scope.cancel_terminate_execution();

        // If the termination is the result of a `Cassie.core.terminate` call, we want
        // to use the exception that was passed to it rather than the exception that
        // was passed to this function.
        exception = state
            .explicit_terminate_exception
            .take()
            .map(|exception| v8::Local::new(scope, exception))
            .unwrap_or_else(|| {
                // Maybe make a new exception object.
                if exception.is_null_or_undefined() {
                    let message = v8::String::new(scope, "execution terminated").unwrap();
                    v8::Exception::error(scope, message)
                } else {
                    exception
                }
            });
    }

    let mut js_error = JsError::from_v8_exception(scope, exception);
    if in_promise {
        js_error.exception_message = format!(
            "Uncaught (in promise) {}",
            js_error.exception_message.trim_start_matches("Uncaught ")
        );
    }
    let js_error = (state.js_error_create_fn)(js_error);

    if is_terminating_exception {
        // Re-enable exception termination.
        scope.terminate_execution();
    }

    Err(js_error)
}

// Related to module loading
impl JsRuntime {
    pub(crate) fn instantiate_module(&mut self, id: ModuleId) -> Result<(), Error> {
        let module_map_rc = Self::module_map(self.v8_isolate());
        let scope = &mut self.handle_scope();
        let tc_scope = &mut v8::TryCatch::new(scope);

        let module = module_map_rc
            .borrow()
            .get_handle(id)
            .map(|handle| v8::Local::new(tc_scope, handle))
            .expect("ModuleInfo not found");

        if module.get_status() == v8::ModuleStatus::Errored {
            let exception = module.get_exception();
            let err = exception_to_err_result(tc_scope, exception, false)
                .map_err(|err| attach_handle_to_error(tc_scope, err, exception));
            return err;
        }

        // IMPORTANT: No borrows to `ModuleMap` can be held at this point because
        // `module_resolve_callback` will be calling into `ModuleMap` from within
        // the isolate.
        let instantiate_result =
            module.instantiate_module(tc_scope, bindings::module_resolve_callback);

        if instantiate_result.is_none() {
            let exception = tc_scope.exception().unwrap();
            let err = exception_to_err_result(tc_scope, exception, false)
                .map_err(|err| attach_handle_to_error(tc_scope, err, exception));
            return err;
        }

        Ok(())
    }

    fn dynamic_import_module_evaluate(
        &mut self,
        load_id: ModuleLoadId,
        id: ModuleId,
    ) -> Result<(), Error> {
        let state_rc = Self::state(self.v8_isolate());
        let module_map_rc = Self::module_map(self.v8_isolate());

        let module_handle = module_map_rc
            .borrow()
            .get_handle(id)
            .expect("ModuleInfo not found");

        let status = {
            let scope = &mut self.handle_scope();
            let module = module_handle.open(scope);
            module.get_status()
        };

        match status {
            v8::ModuleStatus::Instantiated | v8::ModuleStatus::Evaluated => {}
            _ => return Ok(()),
        }

        // IMPORTANT: Top-level-await is enabled, which means that return value
        // of module evaluation is a promise.
        //
        // This promise is internal, and not the same one that gets returned to
        // the user. We add an empty `.catch()` handler so that it does not result
        // in an exception if it rejects. That will instead happen for the other
        // promise if not handled by the user.
        //
        // For more details see:
        // https://v8.dev/features/top-level-await#module-execution-order
        let scope = &mut self.handle_scope();
        let tc_scope = &mut v8::TryCatch::new(scope);
        let module = v8::Local::new(tc_scope, &module_handle);
        let maybe_value = module.evaluate(tc_scope);

        // Update status after evaluating.
        let status = module.get_status();

        if let Some(value) = maybe_value {
            assert!(status == v8::ModuleStatus::Evaluated || status == v8::ModuleStatus::Errored);
            let promise = v8::Local::<v8::Promise>::try_from(value)
                .expect("Expected to get promise as module evaluation result");
            let empty_fn = |_scope: &mut v8::HandleScope,
                            _args: v8::FunctionCallbackArguments,
                            _rv: v8::ReturnValue| {};
            let empty_fn = v8::FunctionTemplate::new(tc_scope, empty_fn);
            let empty_fn = empty_fn.get_function(tc_scope).unwrap();
            promise.catch(tc_scope, empty_fn);
            let mut state = state_rc.borrow_mut();
            let promise_global = v8::Global::new(tc_scope, promise);
            let module_global = v8::Global::new(tc_scope, module);

            let dyn_import_mod_evaluate = DynImportModEvaluate {
                load_id,
                module_id: id,
                promise: promise_global,
                module: module_global,
            };

            state.pending_dyn_mod_evaluate.push(dyn_import_mod_evaluate);
        } else if tc_scope.has_terminated() || tc_scope.is_execution_terminating() {
            return Err(
        generic_error("Cannot evaluate dynamically imported module, because JavaScript execution has been terminated.")
      );
        } else {
            assert!(status == v8::ModuleStatus::Errored);
        }

        Ok(())
    }

    // TODO(bartlomieju): make it return `ModuleEvaluationFuture`?
    /// Evaluates an already instantiated ES module.
    ///
    /// Returns a receiver handle that resolves when module promise resolves.
    /// Implementors must manually call `run_event_loop()` to drive module
    /// evaluation future.
    ///
    /// `Error` can be downcast to a type that exposes additional information
    /// about the V8 exception. By default this type is `JsError`, however it may
    /// be a different type if `RuntimeOptions::js_error_create_fn` has been set.
    ///
    /// This function panics if module has not been instantiated.
    pub fn mod_evaluate(&mut self, id: ModuleId) -> oneshot::Receiver<Result<(), Error>> {
        let state_rc = Self::state(self.v8_isolate());
        let module_map_rc = Self::module_map(self.v8_isolate());
        let scope = &mut self.handle_scope();
        let tc_scope = &mut v8::TryCatch::new(scope);

        let module = module_map_rc
            .borrow()
            .get_handle(id)
            .map(|handle| v8::Local::new(tc_scope, handle))
            .expect("ModuleInfo not found");
        let mut status = module.get_status();
        assert_eq!(status, v8::ModuleStatus::Instantiated);

        let (sender, receiver) = oneshot::channel();

        // IMPORTANT: Top-level-await is enabled, which means that return value
        // of module evaluation is a promise.
        //
        // Because that promise is created internally by V8, when error occurs during
        // module evaluation the promise is rejected, and since the promise has no rejection
        // handler it will result in call to `bindings::promise_reject_callback` adding
        // the promise to pending promise rejection table - meaning JsRuntime will return
        // error on next poll().
        //
        // This situation is not desirable as we want to manually return error at the
        // end of this function to handle it further. It means we need to manually
        // remove this promise from pending promise rejection table.
        //
        // For more details see:
        // https://v8.dev/features/top-level-await#module-execution-order
        let maybe_value = module.evaluate(tc_scope);

        // Update status after evaluating.
        status = module.get_status();

        let explicit_terminate_exception =
            state_rc.borrow_mut().explicit_terminate_exception.take();
        if let Some(exception) = explicit_terminate_exception {
            let exception = v8::Local::new(tc_scope, exception);
            sender
                .send(exception_to_err_result(tc_scope, exception, false))
                .expect("Failed to send module evaluation error.");
        } else if let Some(value) = maybe_value {
            assert!(status == v8::ModuleStatus::Evaluated || status == v8::ModuleStatus::Errored);
            let promise = v8::Local::<v8::Promise>::try_from(value)
                .expect("Expected to get promise as module evaluation result");
            let promise_global = v8::Global::new(tc_scope, promise);
            let mut state = state_rc.borrow_mut();
            state.pending_promise_exceptions.remove(&promise_global);
            let promise_global = v8::Global::new(tc_scope, promise);
            assert!(
                state.pending_mod_evaluate.is_none(),
                "There is already pending top level module evaluation"
            );

            state.pending_mod_evaluate = Some(ModEvaluate {
                promise: promise_global,
                sender,
            });
            tc_scope.perform_microtask_checkpoint();
        } else if tc_scope.has_terminated() || tc_scope.is_execution_terminating() {
            sender
                .send(Err(generic_error(
                    "Cannot evaluate module, because JavaScript execution has been terminated.",
                )))
                .expect("Failed to send module evaluation error.");
        } else {
            assert!(status == v8::ModuleStatus::Errored);
        }

        receiver
    }

    fn dynamic_import_reject(&mut self, id: ModuleLoadId, err: Error) {
        let module_map_rc = Self::module_map(self.v8_isolate());
        let scope = &mut self.handle_scope();

        let resolver_handle = module_map_rc
            .borrow_mut()
            .dynamic_import_map
            .remove(&id)
            .expect("Invalid dynamic import id");
        let resolver = resolver_handle.open(scope);

        let exception = err
            .downcast_ref::<ErrWithV8Handle>()
            .map(|err| err.get_handle(scope))
            .unwrap_or_else(|| {
                let message = err.to_string();
                let message = v8::String::new(scope, &message).unwrap();
                v8::Exception::type_error(scope, message)
            });

        // IMPORTANT: No borrows to `ModuleMap` can be held at this point because
        // rejecting the promise might initiate another `import()` which will
        // in turn call `bindings::host_import_module_dynamically_callback` which
        // will reach into `ModuleMap` from within the isolate.
        resolver.reject(scope, exception).unwrap();
        scope.perform_microtask_checkpoint();
    }

    fn dynamic_import_resolve(&mut self, id: ModuleLoadId, mod_id: ModuleId) {
        let state_rc = Self::state(self.v8_isolate());
        let module_map_rc = Self::module_map(self.v8_isolate());
        let scope = &mut self.handle_scope();

        let resolver_handle = module_map_rc
            .borrow_mut()
            .dynamic_import_map
            .remove(&id)
            .expect("Invalid dynamic import id");
        let resolver = resolver_handle.open(scope);

        let module = {
            module_map_rc
                .borrow()
                .get_handle(mod_id)
                .map(|handle| v8::Local::new(scope, handle))
                .expect("Dyn import module info not found")
        };
        // Resolution success
        assert_eq!(module.get_status(), v8::ModuleStatus::Evaluated);

        // IMPORTANT: No borrows to `ModuleMap` can be held at this point because
        // resolving the promise might initiate another `import()` which will
        // in turn call `bindings::host_import_module_dynamically_callback` which
        // will reach into `ModuleMap` from within the isolate.
        let module_namespace = module.get_module_namespace();
        resolver.resolve(scope, module_namespace).unwrap();
        state_rc.borrow_mut().dyn_module_evaluate_idle_counter = 0;
        scope.perform_microtask_checkpoint();
    }

    fn prepare_dyn_imports(&mut self, cx: &mut Context) -> Poll<Result<(), Error>> {
        let module_map_rc = Self::module_map(self.v8_isolate());

        if module_map_rc.borrow().preparing_dynamic_imports.is_empty() {
            return Poll::Ready(Ok(()));
        }

        loop {
            let poll_result = module_map_rc
                .borrow_mut()
                .preparing_dynamic_imports
                .poll_next_unpin(cx);

            if let Poll::Ready(Some(prepare_poll)) = poll_result {
                let dyn_import_id = prepare_poll.0;
                let prepare_result = prepare_poll.1;

                match prepare_result {
                    Ok(load) => {
                        module_map_rc
                            .borrow_mut()
                            .pending_dynamic_imports
                            .push(load.into_future());
                    }
                    Err(err) => {
                        self.dynamic_import_reject(dyn_import_id, err);
                    }
                }
                // Continue polling for more prepared dynamic imports.
                continue;
            }

            // There are no active dynamic import loads, or none are ready.
            return Poll::Ready(Ok(()));
        }
    }

    fn poll_dyn_imports(&mut self, cx: &mut Context) -> Poll<Result<(), Error>> {
        let module_map_rc = Self::module_map(self.v8_isolate());

        if module_map_rc.borrow().pending_dynamic_imports.is_empty() {
            return Poll::Ready(Ok(()));
        }

        loop {
            let poll_result = module_map_rc
                .borrow_mut()
                .pending_dynamic_imports
                .poll_next_unpin(cx);

            if let Poll::Ready(Some(load_stream_poll)) = poll_result {
                let maybe_result = load_stream_poll.0;
                let mut load = load_stream_poll.1;
                let dyn_import_id = load.id;

                if let Some(load_stream_result) = maybe_result {
                    match load_stream_result {
                        Ok((request, info)) => {
                            // A module (not necessarily the one dynamically imported) has been
                            // fetched. Create and register it, and if successful, poll for the
                            // next recursive-load event related to this dynamic import.
                            let register_result = load.register_and_recurse(
                                &mut self.handle_scope(),
                                &request,
                                &info,
                            );

                            match register_result {
                                Ok(()) => {
                                    // Keep importing until it's fully drained
                                    module_map_rc
                                        .borrow_mut()
                                        .pending_dynamic_imports
                                        .push(load.into_future());
                                }
                                Err(err) => self.dynamic_import_reject(dyn_import_id, err),
                            }
                        }
                        Err(err) => {
                            // A non-javascript error occurred; this could be due to a an invalid
                            // module specifier, or a problem with the source map, or a failure
                            // to fetch the module source code.
                            self.dynamic_import_reject(dyn_import_id, err)
                        }
                    }
                } else {
                    // The top-level module from a dynamic import has been instantiated.
                    // Load is done.
                    let module_id = load.root_module_id.expect("Root module should be loaded");
                    let result = self.instantiate_module(module_id);
                    if let Err(err) = result {
                        self.dynamic_import_reject(dyn_import_id, err);
                    }
                    self.dynamic_import_module_evaluate(dyn_import_id, module_id)?;
                }

                // Continue polling for more ready dynamic imports.
                continue;
            }

            // There are no active dynamic import loads, or none are ready.
            return Poll::Ready(Ok(()));
        }
    }

    /// "cassie_engine" runs V8 with Top Level Await enabled. It means that each
    /// module evaluation returns a promise from V8.
    /// Feature docs: https://v8.dev/features/top-level-await
    ///
    /// This promise resolves after all dependent modules have also
    /// resolved. Each dependent module may perform calls to "import()" and APIs
    /// using async ops will add futures to the runtime's event loop.
    /// It means that the promise returned from module evaluation will
    /// resolve only after all futures in the event loop are done.
    ///
    /// Thus during turn of event loop we need to check if V8 has
    /// resolved or rejected the promise. If the promise is still pending
    /// then another turn of event loop must be performed.
    fn evaluate_pending_module(&mut self) {
        let state_rc = Self::state(self.v8_isolate());

        let maybe_module_evaluation = state_rc.borrow_mut().pending_mod_evaluate.take();

        if maybe_module_evaluation.is_none() {
            return;
        }

        let module_evaluation = maybe_module_evaluation.unwrap();
        let scope = &mut self.handle_scope();

        let promise = module_evaluation.promise.open(scope);
        let promise_state = promise.state();

        match promise_state {
            v8::PromiseState::Pending => {
                // NOTE: `poll_event_loop` will decide if
                // runtime would be woken soon
                state_rc.borrow_mut().pending_mod_evaluate = Some(module_evaluation);
            }
            v8::PromiseState::Fulfilled => {
                scope.perform_microtask_checkpoint();
                // Receiver end might have been already dropped, ignore the result
                let _ = module_evaluation.sender.send(Ok(()));
            }
            v8::PromiseState::Rejected => {
                let exception = promise.result(scope);
                scope.perform_microtask_checkpoint();
                let err1 = exception_to_err_result::<()>(scope, exception, false)
                    .map_err(|err| attach_handle_to_error(scope, err, exception))
                    .unwrap_err();
                // Receiver end might have been already dropped, ignore the result
                let _ = module_evaluation.sender.send(Err(err1));
            }
        }
    }

    fn evaluate_dyn_imports(&mut self) {
        let state_rc = Self::state(self.v8_isolate());
        let mut still_pending = vec![];
        let pending = std::mem::take(&mut state_rc.borrow_mut().pending_dyn_mod_evaluate);
        for pending_dyn_evaluate in pending {
            let maybe_result = {
                let scope = &mut self.handle_scope();

                let module_id = pending_dyn_evaluate.module_id;
                let promise = pending_dyn_evaluate.promise.open(scope);
                let _module = pending_dyn_evaluate.module.open(scope);
                let promise_state = promise.state();

                match promise_state {
                    v8::PromiseState::Pending => {
                        still_pending.push(pending_dyn_evaluate);
                        None
                    }
                    v8::PromiseState::Fulfilled => {
                        Some(Ok((pending_dyn_evaluate.load_id, module_id)))
                    }
                    v8::PromiseState::Rejected => {
                        let exception = promise.result(scope);
                        let err1 = exception_to_err_result::<()>(scope, exception, false)
                            .map_err(|err| attach_handle_to_error(scope, err, exception))
                            .unwrap_err();
                        Some(Err((pending_dyn_evaluate.load_id, err1)))
                    }
                }
            };

            if let Some(result) = maybe_result {
                match result {
                    Ok((dyn_import_id, module_id)) => {
                        self.dynamic_import_resolve(dyn_import_id, module_id);
                    }
                    Err((dyn_import_id, err1)) => {
                        self.dynamic_import_reject(dyn_import_id, err1);
                    }
                }
            }
        }
        state_rc.borrow_mut().pending_dyn_mod_evaluate = still_pending;
    }

    /// Asynchronously load specified module and all of its dependencies.
    ///
    /// The module will be marked as "main", and because of that
    /// "import.meta.main" will return true when checked inside that module.
    ///
    /// User must call `JsRuntime::mod_evaluate` with returned `ModuleId`
    /// manually after load is finished.
    pub async fn load_main_module(
        &mut self,
        specifier: &ModuleSpecifier,
        code: Option<String>,
    ) -> Result<ModuleId, Error> {
        let module_map_rc = Self::module_map(self.v8_isolate());
        if let Some(code) = code {
            module_map_rc.borrow_mut().new_es_module(
                &mut self.handle_scope(),
                // main module
                true,
                specifier.as_str(),
                &code,
            )?;
        }

        let mut load = ModuleMap::load_main(module_map_rc.clone(), specifier.as_str()).await?;

        while let Some(load_result) = load.next().await {
            let (request, info) = load_result?;
            let scope = &mut self.handle_scope();
            load.register_and_recurse(scope, &request, &info)?;
        }

        let root_id = load.root_module_id.expect("Root module should be loaded");
        self.instantiate_module(root_id)?;
        Ok(root_id)
    }

    /// Asynchronously load specified ES module and all of its dependencies.
    ///
    /// This method is meant to be used when loading some utility code that
    /// might be later imported by the main module (ie. an entry point module).
    ///
    /// User must call `JsRuntime::mod_evaluate` with returned `ModuleId`
    /// manually after load is finished.
    pub async fn load_side_module(
        &mut self,
        specifier: &ModuleSpecifier,
        code: Option<String>,
    ) -> Result<ModuleId, Error> {
        let module_map_rc = Self::module_map(self.v8_isolate());
        if let Some(code) = code {
            module_map_rc.borrow_mut().new_es_module(
                &mut self.handle_scope(),
                // not main module
                false,
                specifier.as_str(),
                &code,
            )?;
        }

        let mut load = ModuleMap::load_side(module_map_rc.clone(), specifier.as_str()).await?;

        while let Some(load_result) = load.next().await {
            let (request, info) = load_result?;
            let scope = &mut self.handle_scope();
            load.register_and_recurse(scope, &request, &info)?;
        }

        let root_id = load.root_module_id.expect("Root module should be loaded");
        self.instantiate_module(root_id)?;
        Ok(root_id)
    }

    fn check_promise_exceptions(&mut self) -> Result<(), Error> {
        let state_rc = Self::state(self.v8_isolate());
        let mut state = state_rc.borrow_mut();

        if state.pending_promise_exceptions.is_empty() {
            return Ok(());
        }

        let key = {
            state
                .pending_promise_exceptions
                .keys()
                .next()
                .unwrap()
                .clone()
        };
        let handle = state.pending_promise_exceptions.remove(&key).unwrap();
        drop(state);

        let scope = &mut self.handle_scope();
        let exception = v8::Local::new(scope, handle);
        exception_to_err_result(scope, exception, true)
    }

    // Send finished responses to JS
    fn resolve_async_ops(&mut self, cx: &mut Context) -> Result<(), Error> {
        let state_rc = Self::state(self.v8_isolate());

        let js_recv_cb_handle = state_rc.borrow().js_recv_cb.clone().unwrap();
        let scope = &mut self.handle_scope();

        // We return async responses to JS in unbounded batches (may change),
        // each batch is a flat vector of tuples:
        // `[promise_id1, op_result1, promise_id2, op_result2, ...]`
        // promise_id is a simple integer, op_result is an ops::OpResult
        // which contains a value OR an error, encoded as a tuple.
        // This batch is received in JS via the special `arguments` variable
        // and then each tuple is used to resolve or reject promises
        let mut args: Vec<v8::Local<v8::Value>> = vec![];

        // Now handle actual ops.
        {
            let mut state = state_rc.borrow_mut();
            state.have_unpolled_ops = false;

            while let Poll::Ready(Some(item)) = state.pending_ops.poll_next_unpin(cx) {
                let (promise_id, op_id, resp) = item;
                state.unrefed_ops.remove(&promise_id);
                state.op_state.borrow().tracker.track_async_completed(op_id);
                args.push(v8::Integer::new(scope, promise_id as i32).into());
                args.push(resp.to_v8(scope).unwrap());
            }
        }

        if args.is_empty() {
            return Ok(());
        }

        let tc_scope = &mut v8::TryCatch::new(scope);
        let js_recv_cb = js_recv_cb_handle.open(tc_scope);
        let this = v8::undefined(tc_scope).into();
        js_recv_cb.call(tc_scope, this, args.as_slice());

        match tc_scope.exception() {
            None => Ok(()),
            Some(exception) => exception_to_err_result(tc_scope, exception, false),
        }
    }

    fn drain_macrotasks(&mut self) -> Result<(), Error> {
        let state = Self::state(self.v8_isolate());

        if state.borrow().js_macrotask_cbs.is_empty() {
            return Ok(());
        }

        let js_macrotask_cb_handles = state.borrow().js_macrotask_cbs.clone();
        let scope = &mut self.handle_scope();

        for js_macrotask_cb_handle in js_macrotask_cb_handles {
            let js_macrotask_cb = js_macrotask_cb_handle.open(scope);

            // Repeatedly invoke macrotask callback until it returns true (done),
            // such that ready microtasks would be automatically run before
            // next macrotask is processed.
            let tc_scope = &mut v8::TryCatch::new(scope);
            let this = v8::undefined(tc_scope).into();
            loop {
                let is_done = js_macrotask_cb.call(tc_scope, this, &[]);

                if let Some(exception) = tc_scope.exception() {
                    return exception_to_err_result(tc_scope, exception, false);
                }

                if tc_scope.has_terminated() || tc_scope.is_execution_terminating() {
                    return Ok(());
                }

                let is_done = is_done.unwrap();
                if is_done.is_true() {
                    break;
                }
            }
        }

        Ok(())
    }

    fn drain_nexttick(&mut self) -> Result<(), Error> {
        let state = Self::state(self.v8_isolate());

        if state.borrow().js_nexttick_cbs.is_empty() {
            return Ok(());
        }

        if !state.borrow().has_tick_scheduled {
            let scope = &mut self.handle_scope();
            scope.perform_microtask_checkpoint();
        }

        // TODO(bartlomieju): Node also checks for absence of "rejection_to_warn"
        if !state.borrow().has_tick_scheduled {
            return Ok(());
        }

        let js_nexttick_cb_handles = state.borrow().js_nexttick_cbs.clone();
        let scope = &mut self.handle_scope();

        for js_nexttick_cb_handle in js_nexttick_cb_handles {
            let js_nexttick_cb = js_nexttick_cb_handle.open(scope);

            let tc_scope = &mut v8::TryCatch::new(scope);
            let this = v8::undefined(tc_scope).into();
            js_nexttick_cb.call(tc_scope, this, &[]);

            if let Some(exception) = tc_scope.exception() {
                return exception_to_err_result(tc_scope, exception, false);
            }
        }

        Ok(())
    }
}

#[inline]
pub fn queue_async_op(
    scope: &v8::Isolate,
    op: impl Future<Output = (PromiseId, OpId, OpResult)> + 'static,
) {
    let state_rc = JsRuntime::state(scope);
    let mut state = state_rc.borrow_mut();
    state.pending_ops.push(OpCall::eager(op));
    state.have_unpolled_ops = true;
}