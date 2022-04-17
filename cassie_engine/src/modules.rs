use crate::bindings;
use crate::error::attach_handle_to_error;
use crate::error::generic_error;
use crate::module_specifier::resolve_url;
use crate::module_specifier::ModuleSpecifier;
use crate::ops::OpState;
use crate::resolve_import;
use crate::runtime::exception_to_err_result;
use anyhow::Error;
use futures::future::FutureExt;
use futures::stream::FuturesUnordered;
use futures::stream::Stream;
use futures::stream::StreamFuture;
use futures::stream::TryStreamExt;
use log::debug;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::Context;
use std::task::Poll;

pub type ModuleId = i32;
pub(crate) type ModuleLoadId = i32;

pub const BOM_CHAR: char = '\u{FEFF}';

/// Strips the byte order mark from the provided text if it exists.
fn strip_bom(text: &str) -> &str {
    if text.starts_with(BOM_CHAR) {
        &text[BOM_CHAR.len_utf8()..]
    } else {
        text
    }
}

const SUPPORTED_TYPE_ASSERTIONS: &[&str] = &["json"];

/// Throws V8 exception if assertions are invalid
pub(crate) fn validate_import_assertions(
    scope: &mut v8::HandleScope,
    assertions: &HashMap<String, String>,
) {
    for (key, value) in assertions {
        if key == "type" && !SUPPORTED_TYPE_ASSERTIONS.contains(&value.as_str()) {
            let message =
                v8::String::new(scope, &format!("\"{}\" is not a valid module type.", value))
                    .unwrap();
            let exception = v8::Exception::type_error(scope, message);
            scope.throw_exception(exception);
            return;
        }
    }
}

#[derive(Debug)]
pub(crate) enum ImportAssertionsKind {
    StaticImport,
    DynamicImport,
}

pub(crate) fn parse_import_assertions(
    scope: &mut v8::HandleScope,
    import_assertions: v8::Local<v8::FixedArray>,
    kind: ImportAssertionsKind,
) -> HashMap<String, String> {
    let mut assertions: HashMap<String, String> = HashMap::default();

    let assertions_per_line = match kind {
        // For static imports, assertions are triples of (keyword, value and source offset)
        // Also used in `module_resolve_callback`.
        ImportAssertionsKind::StaticImport => 3,
        // For dynamic imports, assertions are tuples of (keyword, value)
        ImportAssertionsKind::DynamicImport => 2,
    };
    assert_eq!(import_assertions.length() % assertions_per_line, 0);
    let no_of_assertions = import_assertions.length() / assertions_per_line;

    for i in 0..no_of_assertions {
        let assert_key = import_assertions
            .get(scope, assertions_per_line * i)
            .unwrap();
        let assert_key_val = v8::Local::<v8::Value>::try_from(assert_key).unwrap();
        let assert_value = import_assertions
            .get(scope, (assertions_per_line * i) + 1)
            .unwrap();
        let assert_value_val = v8::Local::<v8::Value>::try_from(assert_value).unwrap();
        assertions.insert(
            assert_key_val.to_rust_string_lossy(scope),
            assert_value_val.to_rust_string_lossy(scope),
        );
    }

    assertions
}

pub(crate) fn get_module_type_from_assertions(assertions: &HashMap<String, String>) -> ModuleType {
    assertions
        .get("type")
        .map(|ty| {
            if ty == "json" {
                ModuleType::Json
            } else {
                ModuleType::JavaScript
            }
        })
        .unwrap_or(ModuleType::JavaScript)
}

// Clippy thinks the return value doesn't need to be an Option, it's unaware
// of the mapping that MapFnFrom<F> does for ResolveModuleCallback.
#[allow(clippy::unnecessary_wraps)]
fn json_module_evaluation_steps<'a>(
    context: v8::Local<'a, v8::Context>,
    module: v8::Local<v8::Module>,
) -> Option<v8::Local<'a, v8::Value>> {
    let scope = &mut unsafe { v8::CallbackScope::new(context) };
    let tc_scope = &mut v8::TryCatch::new(scope);
    let module_map = tc_scope
        .get_slot::<Rc<RefCell<ModuleMap>>>()
        .unwrap()
        .clone();

    let handle = v8::Global::<v8::Module>::new(tc_scope, module);
    let value_handle = module_map
        .borrow_mut()
        .json_value_store
        .remove(&handle)
        .unwrap();
    let value_local = v8::Local::new(tc_scope, value_handle);

    let name = v8::String::new(tc_scope, "default").unwrap();
    // This should never fail
    assert!(module.set_synthetic_module_export(tc_scope, name, value_local) == Some(true));
    assert!(!tc_scope.has_caught());

    // Since TLA is active we need to return a promise.
    let resolver = v8::PromiseResolver::new(tc_scope).unwrap();
    let undefined = v8::undefined(tc_scope);
    resolver.resolve(tc_scope, undefined.into());
    Some(resolver.get_promise(tc_scope).into())
}

/// A type of module to be executed.
///
/// For non-`JavaScript` modules, this value doesn't tell
/// how to interpret the module; it is only used to validate
/// the module against an import assertion (if one is present
/// in the import statement).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ModuleType {
    JavaScript,
    Json,
}

impl std::fmt::Display for ModuleType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::JavaScript => write!(f, "JavaScript"),
            Self::Json => write!(f, "JSON"),
        }
    }
}

/// EsModule source code that will be loaded into V8.
///
/// Users can implement `Into<ModuleInfo>` for different file types that
/// can be transpiled to valid EsModule.
///
/// Found module URL might be different from specified URL
/// used for loading due to redirections (like HTTP 303).
/// Eg. Both "`https://example.com/a.ts`" and
/// "`https://example.com/b.ts`" may point to "`https://example.com/c.ts`"
/// By keeping track of specified and found URL we can alias modules and avoid
/// recompiling the same code 3 times.
// TODO(bartlomieju): I have a strong opinion we should store all redirects
// that happened; not only first and final target. It would simplify a lot
// of things throughout the codebase otherwise we may end up requesting
// intermediate redirects from file loader.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ModuleSource {
    pub code: String,
    pub module_type: ModuleType,
    pub module_url_specified: String,
    pub module_url_found: String,
}

pub(crate) type PrepareLoadFuture =
    dyn Future<Output = (ModuleLoadId, Result<RecursiveModuleLoad, Error>)>;
pub type ModuleSourceFuture = dyn Future<Output = Result<ModuleSource, Error>>;

type ModuleLoadFuture = dyn Future<Output = Result<(ModuleRequest, ModuleSource), Error>>;

pub trait ModuleLoader {
    /// Returns an absolute URL.
    /// When implementing an spec-complaint VM, this should be exactly the
    /// algorithm described here:
    /// <https://html.spec.whatwg.org/multipage/webappapis.html#resolve-a-module-specifier>
    ///
    /// `is_main` can be used to resolve from current working directory or
    /// apply import map for child imports.
    fn resolve(
        &self,
        specifier: &str,
        referrer: &str,
        _is_main: bool,
    ) -> Result<ModuleSpecifier, Error>;

    /// Given ModuleSpecifier, load its source code.
    ///
    /// `is_dyn_import` can be used to check permissions or deny
    /// dynamic imports altogether.
    fn load(
        &self,
        module_specifier: &ModuleSpecifier,
        maybe_referrer: Option<ModuleSpecifier>,
        is_dyn_import: bool,
    ) -> Pin<Box<ModuleSourceFuture>>;

    /// This hook can be used by implementors to do some preparation
    /// work before starting loading of modules.
    ///
    /// For example implementor might download multiple modules in
    /// parallel and transpile them to final JS sources before
    /// yielding control back to the runtime.
    ///
    /// It's not required to implement this method.
    fn prepare_load(
        &self,
        _op_state: Rc<RefCell<OpState>>,
        _module_specifier: &ModuleSpecifier,
        _maybe_referrer: Option<String>,
        _is_dyn_import: bool,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>>>> {
        async { Ok(()) }.boxed_local()
    }
}

/// Placeholder structure used when creating
/// a runtime that doesn't support module loading.
pub struct NoopModuleLoader;

impl ModuleLoader for NoopModuleLoader {
    fn resolve(
        &self,
        _specifier: &str,
        _referrer: &str,
        _is_main: bool,
    ) -> Result<ModuleSpecifier, Error> {
        Err(generic_error("Module loading is not supported"))
    }

    fn load(
        &self,
        _module_specifier: &ModuleSpecifier,
        _maybe_referrer: Option<ModuleSpecifier>,
        _is_dyn_import: bool,
    ) -> Pin<Box<ModuleSourceFuture>> {
        async { Err(generic_error("Module loading is not supported")) }.boxed_local()
    }
}

/// Basic file system module loader.
///
/// Note that this loader will **block** event loop
/// when loading file as it uses synchronous FS API
/// from standard library.
pub struct FsModuleLoader;

impl ModuleLoader for FsModuleLoader {
    fn resolve(
        &self,
        specifier: &str,
        referrer: &str,
        _is_main: bool,
    ) -> Result<ModuleSpecifier, Error> {
        Ok(resolve_import(specifier, referrer)?)
    }

    fn load(
        &self,
        module_specifier: &ModuleSpecifier,
        _maybe_referrer: Option<ModuleSpecifier>,
        _is_dynamic: bool,
    ) -> Pin<Box<ModuleSourceFuture>> {
        let module_specifier = module_specifier.clone();
        async move {
            let path = module_specifier.to_file_path().map_err(|_| {
                generic_error(format!(
                    "Provided module specifier \"{}\" is not a file URL.",
                    module_specifier
                ))
            })?;
            let module_type = if let Some(extension) = path.extension() {
                let ext = extension.to_string_lossy().to_lowercase();
                if ext == "json" {
                    ModuleType::Json
                } else {
                    ModuleType::JavaScript
                }
            } else {
                ModuleType::JavaScript
            };

            let code = std::fs::read_to_string(path)?;
            let module = ModuleSource {
                code,
                module_type,
                module_url_specified: module_specifier.to_string(),
                module_url_found: module_specifier.to_string(),
            };
            Ok(module)
        }
        .boxed_local()
    }
}

/// Describes the entrypoint of a recursive module load.
#[derive(Debug)]
enum LoadInit {
    /// Main module specifier.
    Main(String),
    /// Module specifier for side module.
    Side(String),
    /// Dynamic import specifier with referrer and expected
    /// module type (which is determined by import assertion).
    DynamicImport(String, String, ModuleType),
}

#[derive(Debug, Eq, PartialEq)]
pub enum LoadState {
    Init,
    LoadingRoot,
    LoadingImports,
    Done,
}

/// This future is used to implement parallel async module loading.
pub(crate) struct RecursiveModuleLoad {
    pub id: ModuleLoadId,
    pub root_module_id: Option<ModuleId>,
    init: LoadInit,
    root_module_type: Option<ModuleType>,
    state: LoadState,
    module_map_rc: Rc<RefCell<ModuleMap>>,
    pending: FuturesUnordered<Pin<Box<ModuleLoadFuture>>>,
    visited: HashSet<ModuleRequest>,
    // These two fields are copied from `module_map_rc`, but they are cloned ahead
    // of time to avoid already-borrowed errors.
    op_state: Rc<RefCell<OpState>>,
    loader: Rc<dyn ModuleLoader>,
}

impl RecursiveModuleLoad {
    /// Starts a new asynchronous load of the module graph for given specifier.
    ///
    /// The module corresponding for the given `specifier` will be marked as
    // "the main module" (`import.meta.main` will return `true` for this module).
    fn main(specifier: &str, module_map_rc: Rc<RefCell<ModuleMap>>) -> Self {
        Self::new(LoadInit::Main(specifier.to_string()), module_map_rc)
    }

    /// Starts a new asynchronous load of the module graph for given specifier.
    fn side(specifier: &str, module_map_rc: Rc<RefCell<ModuleMap>>) -> Self {
        Self::new(LoadInit::Side(specifier.to_string()), module_map_rc)
    }

    /// Starts a new asynchronous load of the module graph for given specifier
    /// that was imported using `import()`.
    fn dynamic_import(
        specifier: &str,
        referrer: &str,
        module_type: ModuleType,
        module_map_rc: Rc<RefCell<ModuleMap>>,
    ) -> Self {
        Self::new(
            LoadInit::DynamicImport(specifier.to_string(), referrer.to_string(), module_type),
            module_map_rc,
        )
    }

    fn new(init: LoadInit, module_map_rc: Rc<RefCell<ModuleMap>>) -> Self {
        let id = {
            let mut module_map = module_map_rc.borrow_mut();
            let id = module_map.next_load_id;
            module_map.next_load_id += 1;
            id
        };
        let op_state = module_map_rc.borrow().op_state.clone();
        let loader = module_map_rc.borrow().loader.clone();
        let expected_module_type = match init {
            LoadInit::DynamicImport(_, _, module_type) => module_type,
            _ => ModuleType::JavaScript,
        };
        let mut load = Self {
            id,
            root_module_id: None,
            root_module_type: None,
            init,
            state: LoadState::Init,
            module_map_rc: module_map_rc.clone(),
            op_state,
            loader,
            pending: FuturesUnordered::new(),
            visited: HashSet::new(),
        };
        // FIXME(bartlomieju): this seems fishy
        // Ignore the error here, let it be hit in `Stream::poll_next()`.
        if let Ok(root_specifier) = load.resolve_root() {
            if let Some(module_id) = module_map_rc
                .borrow()
                .get_id(root_specifier.as_str(), expected_module_type)
            {
                load.root_module_id = Some(module_id);
                load.root_module_type = Some(expected_module_type);
            }
        }
        load
    }

    fn resolve_root(&self) -> Result<ModuleSpecifier, Error> {
        match self.init {
            LoadInit::Main(ref specifier) => self.loader.resolve(specifier, ".", true),
            LoadInit::Side(ref specifier) => self.loader.resolve(specifier, ".", false),
            LoadInit::DynamicImport(ref specifier, ref referrer, _) => {
                self.loader.resolve(specifier, referrer, false)
            }
        }
    }

    async fn prepare(&self) -> Result<(), Error> {
        let op_state = self.op_state.clone();
        let (module_specifier, maybe_referrer) = match self.init {
            LoadInit::Main(ref specifier) => {
                let spec = self.loader.resolve(specifier, ".", true)?;
                (spec, None)
            }
            LoadInit::Side(ref specifier) => {
                let spec = self.loader.resolve(specifier, ".", false)?;
                (spec, None)
            }
            LoadInit::DynamicImport(ref specifier, ref referrer, _) => {
                let spec = self.loader.resolve(specifier, referrer, false)?;
                (spec, Some(referrer.to_string()))
            }
        };

        self.loader
            .prepare_load(
                op_state,
                &module_specifier,
                maybe_referrer,
                self.is_dynamic_import(),
            )
            .await
    }

    fn is_currently_loading_main_module(&self) -> bool {
        !self.is_dynamic_import()
            && matches!(self.init, LoadInit::Main(..))
            && self.state == LoadState::LoadingRoot
    }

    fn is_dynamic_import(&self) -> bool {
        matches!(self.init, LoadInit::DynamicImport(..))
    }

    pub(crate) fn register_and_recurse(
        &mut self,
        scope: &mut v8::HandleScope,
        module_request: &ModuleRequest,
        module_source: &ModuleSource,
    ) -> Result<(), Error> {
        if module_request.expected_module_type != module_source.module_type {
            return Err(generic_error(format!(
                "Expected a \"{}\" module but loaded a \"{}\" module.",
                module_request.expected_module_type, module_source.module_type,
            )));
        }

        // Register the module in the module map unless it's already there. If the
        // specified URL and the "true" URL are different, register the alias.
        if module_source.module_url_specified != module_source.module_url_found {
            self.module_map_rc.borrow_mut().alias(
                &module_source.module_url_specified,
                module_source.module_type,
                &module_source.module_url_found,
            );
        }
        let maybe_module_id = self
            .module_map_rc
            .borrow()
            .get_id(&module_source.module_url_found, module_source.module_type);
        let module_id = match maybe_module_id {
            Some(id) => {
                debug!(
                    "Already-registered module fetched again: {}",
                    module_source.module_url_found
                );
                id
            }
            None => match module_source.module_type {
                ModuleType::JavaScript => self.module_map_rc.borrow_mut().new_es_module(
                    scope,
                    self.is_currently_loading_main_module(),
                    &module_source.module_url_found,
                    &module_source.code,
                )?,
                ModuleType::Json => self.module_map_rc.borrow_mut().new_json_module(
                    scope,
                    &module_source.module_url_found,
                    &module_source.code,
                )?,
            },
        };

        // Recurse the module's imports. There are two cases for each import:
        // 1. If the module is not in the module map, start a new load for it in
        //    `self.pending`. The result of that load should eventually be passed to
        //    this function for recursion.
        // 2. If the module is already in the module map, queue it up to be
        //    recursed synchronously here.
        // This robustly ensures that the whole graph is in the module map before
        // `LoadState::Done` is set.
        let mut already_registered = VecDeque::new();
        already_registered.push_back((module_id, module_request.clone()));
        self.visited.insert(module_request.clone());
        while let Some((module_id, module_request)) = already_registered.pop_front() {
            let referrer = module_request.specifier.clone();
            let imports = self
                .module_map_rc
                .borrow()
                .get_requested_modules(module_id)
                .unwrap()
                .clone();
            for module_request in imports {
                if !self.visited.contains(&module_request) {
                    if let Some(module_id) = self.module_map_rc.borrow().get_id(
                        module_request.specifier.as_str(),
                        module_request.expected_module_type,
                    ) {
                        already_registered.push_back((module_id, module_request.clone()));
                    } else {
                        let referrer = referrer.clone();
                        let request = module_request.clone();
                        let loader = self.loader.clone();
                        let is_dynamic_import = self.is_dynamic_import();
                        let fut = async move {
                            let load_result = loader
                                .load(
                                    &request.specifier,
                                    Some(referrer.clone()),
                                    is_dynamic_import,
                                )
                                .await;
                            load_result.map(|s| (request, s))
                        };
                        self.pending.push(fut.boxed_local());
                    }
                    self.visited.insert(module_request);
                }
            }
        }

        // Update `self.state` however applicable.
        if self.state == LoadState::LoadingRoot {
            self.root_module_id = Some(module_id);
            self.root_module_type = Some(module_source.module_type);
            self.state = LoadState::LoadingImports;
        }
        if self.pending.is_empty() {
            self.state = LoadState::Done;
        }

        Ok(())
    }
}

impl Stream for RecursiveModuleLoad {
    type Item = Result<(ModuleRequest, ModuleSource), Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let inner = self.get_mut();
        // IMPORTANT: Do not borrow `inner.module_map_rc` here. It may not be
        // available.
        match inner.state {
            LoadState::Init => {
                let module_specifier = match inner.resolve_root() {
                    Ok(url) => url,
                    Err(error) => return Poll::Ready(Some(Err(error))),
                };
                let load_fut = if let Some(_module_id) = inner.root_module_id {
                    // FIXME(bartlomieju): this is very bad
                    // The root module is already in the module map.
                    // TODO(nayeemrmn): In this case we would ideally skip to
                    // `LoadState::LoadingImports` and synchronously recurse the imports
                    // like the bottom of `RecursiveModuleLoad::register_and_recurse()`.
                    // But the module map cannot be borrowed here. Instead fake a load
                    // event so it gets passed to that function and recursed eventually.
                    let module_type = inner.root_module_type.unwrap();
                    let module_request = ModuleRequest {
                        specifier: module_specifier.clone(),
                        expected_module_type: module_type,
                    };
                    let module_source = ModuleSource {
                        module_url_specified: module_specifier.to_string(),
                        module_url_found: module_specifier.to_string(),
                        // The code will be discarded, since this module is already in the
                        // module map.
                        code: Default::default(),
                        module_type,
                    };
                    futures::future::ok((module_request, module_source)).boxed()
                } else {
                    let maybe_referrer = match inner.init {
                        LoadInit::DynamicImport(_, ref referrer, _) => resolve_url(referrer).ok(),
                        _ => None,
                    };
                    let expected_module_type = match inner.init {
                        LoadInit::DynamicImport(_, _, module_type) => module_type,
                        _ => ModuleType::JavaScript,
                    };
                    let module_request = ModuleRequest {
                        specifier: module_specifier.clone(),
                        expected_module_type,
                    };
                    let loader = inner.loader.clone();
                    let is_dynamic_import = inner.is_dynamic_import();
                    async move {
                        let result = loader
                            .load(&module_specifier, maybe_referrer, is_dynamic_import)
                            .await;
                        result.map(|s| (module_request, s))
                    }
                    .boxed_local()
                };
                inner.pending.push(load_fut);
                inner.state = LoadState::LoadingRoot;
                inner.try_poll_next_unpin(cx)
            }
            LoadState::LoadingRoot | LoadState::LoadingImports => {
                match inner.pending.try_poll_next_unpin(cx)? {
                    Poll::Ready(None) => unreachable!(),
                    Poll::Ready(Some(info)) => Poll::Ready(Some(Ok(info))),
                    Poll::Pending => Poll::Pending,
                }
            }
            LoadState::Done => Poll::Ready(None),
        }
    }
}

/// Describes what is the expected type of module, usually
/// it's `ModuleType::JavaScript`, but if there were import assertions
/// it might be `ModuleType::Json`.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct ModuleRequest {
    pub specifier: ModuleSpecifier,
    pub expected_module_type: ModuleType,
}

pub(crate) struct ModuleInfo {
    #[allow(unused)]
    pub id: ModuleId,
    // Used in "bindings.rs" for "import.meta.main" property value.
    pub main: bool,
    pub name: String,
    pub requests: Vec<ModuleRequest>,
    pub module_type: ModuleType,
}

/// A symbolic module entity.
enum SymbolicModule {
    /// This module is an alias to another module.
    /// This is useful such that multiple names could point to
    /// the same underlying module (particularly due to redirects).
    Alias(String),
    /// This module associates with a V8 module by id.
    Mod(ModuleId),
}

/// A collection of JS modules.
pub(crate) struct ModuleMap {
    // Handling of specifiers and v8 objects
    ids_by_handle: HashMap<v8::Global<v8::Module>, ModuleId>,
    handles_by_id: HashMap<ModuleId, v8::Global<v8::Module>>,
    info: HashMap<ModuleId, ModuleInfo>,
    by_name: HashMap<(String, ModuleType), SymbolicModule>,
    next_module_id: ModuleId,
    next_load_id: ModuleLoadId,

    // Handling of futures for loading module sources
    pub loader: Rc<dyn ModuleLoader>,
    op_state: Rc<RefCell<OpState>>,
    pub(crate) dynamic_import_map: HashMap<ModuleLoadId, v8::Global<v8::PromiseResolver>>,
    pub(crate) preparing_dynamic_imports: FuturesUnordered<Pin<Box<PrepareLoadFuture>>>,
    pub(crate) pending_dynamic_imports: FuturesUnordered<StreamFuture<RecursiveModuleLoad>>,

    // This store is used temporarly, to forward parsed JSON
    // value from `new_json_module` to `json_module_evaluation_steps`
    json_value_store: HashMap<v8::Global<v8::Module>, v8::Global<v8::Value>>,
}

impl ModuleMap {
    pub(crate) fn new(loader: Rc<dyn ModuleLoader>, op_state: Rc<RefCell<OpState>>) -> ModuleMap {
        Self {
            ids_by_handle: HashMap::new(),
            handles_by_id: HashMap::new(),
            info: HashMap::new(),
            by_name: HashMap::new(),
            next_module_id: 1,
            next_load_id: 1,
            loader,
            op_state,
            dynamic_import_map: HashMap::new(),
            preparing_dynamic_imports: FuturesUnordered::new(),
            pending_dynamic_imports: FuturesUnordered::new(),
            json_value_store: HashMap::new(),
        }
    }

    /// Get module id, following all aliases in case of module specifier
    /// that had been redirected.
    fn get_id(&self, name: &str, module_type: ModuleType) -> Option<ModuleId> {
        let mut mod_name = name;
        loop {
            let symbolic_module = self.by_name.get(&(mod_name.to_string(), module_type))?;
            match symbolic_module {
                SymbolicModule::Alias(target) => {
                    mod_name = target;
                }
                SymbolicModule::Mod(mod_id) => return Some(*mod_id),
            }
        }
    }

    fn new_json_module(
        &mut self,
        scope: &mut v8::HandleScope,
        name: &str,
        source: &str,
    ) -> Result<ModuleId, Error> {
        let name_str = v8::String::new(scope, name).unwrap();
        let source_str = v8::String::new(scope, strip_bom(source)).unwrap();

        let tc_scope = &mut v8::TryCatch::new(scope);

        let parsed_json = match v8::json::parse(tc_scope, source_str) {
            Some(parsed_json) => parsed_json,
            None => {
                assert!(tc_scope.has_caught());
                let exception = tc_scope.exception().unwrap();
                let err = exception_to_err_result(tc_scope, exception, false)
                    .map_err(|err| attach_handle_to_error(tc_scope, err, exception));
                return err;
            }
        };

        let export_names = [v8::String::new(tc_scope, "default").unwrap()];
        let module = v8::Module::create_synthetic_module(
            tc_scope,
            name_str,
            &export_names,
            json_module_evaluation_steps,
        );

        let handle = v8::Global::<v8::Module>::new(tc_scope, module);
        let value_handle = v8::Global::<v8::Value>::new(tc_scope, parsed_json);
        self.json_value_store.insert(handle.clone(), value_handle);

        let id = self.create_module_info(name, ModuleType::Json, handle, false, vec![]);

        Ok(id)
    }

    // Create and compile an ES module.
    pub(crate) fn new_es_module(
        &mut self,
        scope: &mut v8::HandleScope,
        main: bool,
        name: &str,
        source: &str,
    ) -> Result<ModuleId, Error> {
        let name_str = v8::String::new(scope, name).unwrap();
        let source_str = v8::String::new(scope, source).unwrap();

        let origin = bindings::module_origin(scope, name_str);
        let source = v8::script_compiler::Source::new(source_str, Some(&origin));

        let tc_scope = &mut v8::TryCatch::new(scope);

        let maybe_module = v8::script_compiler::compile_module(tc_scope, source);

        if tc_scope.has_caught() {
            assert!(maybe_module.is_none());
            let e = tc_scope.exception().unwrap();
            return exception_to_err_result(tc_scope, e, false);
        }

        let module = maybe_module.unwrap();

        let mut requests: Vec<ModuleRequest> = vec![];
        let module_requests = module.get_module_requests();
        for i in 0..module_requests.length() {
            let module_request =
                v8::Local::<v8::ModuleRequest>::try_from(module_requests.get(tc_scope, i).unwrap())
                    .unwrap();
            let import_specifier = module_request
                .get_specifier()
                .to_rust_string_lossy(tc_scope);

            let import_assertions = module_request.get_import_assertions();

            let assertions = parse_import_assertions(
                tc_scope,
                import_assertions,
                ImportAssertionsKind::StaticImport,
            );

            // FIXME(bartomieju): there are no stack frames if exception
            // is thrown here
            validate_import_assertions(tc_scope, &assertions);
            if tc_scope.has_caught() {
                let e = tc_scope.exception().unwrap();
                return exception_to_err_result(tc_scope, e, false);
            }

            let module_specifier = self.loader.resolve(&import_specifier, name, false)?;
            let expected_module_type = get_module_type_from_assertions(&assertions);
            let request = ModuleRequest {
                specifier: module_specifier,
                expected_module_type,
            };
            requests.push(request);
        }

        if main {
            let maybe_main_module = self.info.values().find(|module| module.main);
            if let Some(main_module) = maybe_main_module {
                return Err(generic_error(format!(
                    "Trying to create \"main\" module ({:?}), when one already exists ({:?})",
                    name, main_module.name,
                )));
            }
        }

        let handle = v8::Global::<v8::Module>::new(tc_scope, module);
        let id = self.create_module_info(name, ModuleType::JavaScript, handle, main, requests);

        Ok(id)
    }

    fn create_module_info(
        &mut self,
        name: &str,
        module_type: ModuleType,
        handle: v8::Global<v8::Module>,
        main: bool,
        requests: Vec<ModuleRequest>,
    ) -> ModuleId {
        let id = self.next_module_id;
        self.next_module_id += 1;
        self.by_name
            .insert((name.to_string(), module_type), SymbolicModule::Mod(id));
        self.handles_by_id.insert(id, handle.clone());
        self.ids_by_handle.insert(handle, id);
        self.info.insert(
            id,
            ModuleInfo {
                id,
                main,
                name: name.to_string(),
                requests,
                module_type,
            },
        );

        id
    }

    fn get_requested_modules(&self, id: ModuleId) -> Option<&Vec<ModuleRequest>> {
        self.info.get(&id).map(|i| &i.requests)
    }

    fn is_registered(&self, specifier: &ModuleSpecifier, module_type: ModuleType) -> bool {
        if let Some(id) = self.get_id(specifier.as_str(), module_type) {
            let info = self.get_info_by_id(&id).unwrap();
            return info.module_type == module_type;
        }

        false
    }

    fn alias(&mut self, name: &str, module_type: ModuleType, target: &str) {
        self.by_name.insert(
            (name.to_string(), module_type),
            SymbolicModule::Alias(target.to_string()),
        );
    }

    #[cfg(test)]
    fn is_alias(&self, name: &str, module_type: ModuleType) -> bool {
        let cond = self.by_name.get(&(name.to_string(), module_type));
        matches!(cond, Some(SymbolicModule::Alias(_)))
    }

    pub(crate) fn get_handle(&self, id: ModuleId) -> Option<v8::Global<v8::Module>> {
        self.handles_by_id.get(&id).cloned()
    }

    pub(crate) fn get_info(&self, global: &v8::Global<v8::Module>) -> Option<&ModuleInfo> {
        if let Some(id) = self.ids_by_handle.get(global) {
            return self.info.get(id);
        }

        None
    }

    pub(crate) fn get_info_by_id(&self, id: &ModuleId) -> Option<&ModuleInfo> {
        self.info.get(id)
    }

    pub(crate) async fn load_main(
        module_map_rc: Rc<RefCell<ModuleMap>>,
        specifier: &str,
    ) -> Result<RecursiveModuleLoad, Error> {
        let load = RecursiveModuleLoad::main(specifier, module_map_rc.clone());
        load.prepare().await?;
        Ok(load)
    }

    pub(crate) async fn load_side(
        module_map_rc: Rc<RefCell<ModuleMap>>,
        specifier: &str,
    ) -> Result<RecursiveModuleLoad, Error> {
        let load = RecursiveModuleLoad::side(specifier, module_map_rc.clone());
        load.prepare().await?;
        Ok(load)
    }

    // Initiate loading of a module graph imported using `import()`.
    pub(crate) fn load_dynamic_import(
        module_map_rc: Rc<RefCell<ModuleMap>>,
        specifier: &str,
        referrer: &str,
        module_type: ModuleType,
        resolver_handle: v8::Global<v8::PromiseResolver>,
    ) {
        let load = RecursiveModuleLoad::dynamic_import(
            specifier,
            referrer,
            module_type,
            module_map_rc.clone(),
        );
        module_map_rc
            .borrow_mut()
            .dynamic_import_map
            .insert(load.id, resolver_handle);
        let resolve_result = module_map_rc
            .borrow()
            .loader
            .resolve(specifier, referrer, false);
        let fut = match resolve_result {
            Ok(module_specifier) => {
                if module_map_rc
                    .borrow()
                    .is_registered(&module_specifier, module_type)
                {
                    async move { (load.id, Ok(load)) }.boxed_local()
                } else {
                    async move { (load.id, load.prepare().await.map(|()| load)) }.boxed_local()
                }
            }
            Err(error) => async move { (load.id, Err(error)) }.boxed_local(),
        };
        module_map_rc
            .borrow_mut()
            .preparing_dynamic_imports
            .push(fut);
    }

    pub(crate) fn has_pending_dynamic_imports(&self) -> bool {
        !(self.preparing_dynamic_imports.is_empty() && self.pending_dynamic_imports.is_empty())
    }

    /// Called by `module_resolve_callback` during module instantiation.
    pub(crate) fn resolve_callback<'s>(
        &self,
        scope: &mut v8::HandleScope<'s>,
        specifier: &str,
        referrer: &str,
        import_assertions: HashMap<String, String>,
    ) -> Option<v8::Local<'s, v8::Module>> {
        let resolved_specifier = self
            .loader
            .resolve(specifier, referrer, false)
            .expect("Module should have been already resolved");

        let module_type = get_module_type_from_assertions(&import_assertions);

        if let Some(id) = self.get_id(resolved_specifier.as_str(), module_type) {
            if let Some(handle) = self.get_handle(id) {
                return Some(v8::Local::new(scope, handle));
            }
        }

        None
    }
}
