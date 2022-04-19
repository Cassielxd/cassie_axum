use crate::service::sys_dict_service::get_all_list;
use cassie_domain::dto::sys_dict_dto::SysDictTypeDTO;
use deno_core::JsRuntime;
use deno_core::RuntimeOptions;
use deno_runtime::deno_broadcast_channel::InMemoryBroadcastChannel;
use deno_runtime::deno_core::error::AnyError;
use deno_runtime::deno_core::op;
use deno_runtime::deno_core::Extension;
use deno_runtime::deno_core::FsModuleLoader;
use deno_runtime::deno_web::BlobStore;
use deno_runtime::permissions::Permissions;
use deno_runtime::worker::MainWorker;
use deno_runtime::worker::WorkerOptions;
use deno_runtime::{deno_core, BootstrapOptions,js::deno_isolate_init};
use log::info;
use tokio::time::Instant;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
//初始话规则引擎
pub fn init_rules() {
    test1()
}

fn get_workers()->Arc<Mutex<MainWorker>>{
    static mut POINT: Option<Arc<Mutex<MainWorker>>> = None;
    unsafe {// Rust中使用可变静态变量都是unsafe的
        POINT.get_or_insert_with(|| {
            // 初始化单例对象的代码
            Arc::new(Mutex::new(init()))
        }).clone()
    }
}
//默认使用 最小化的模式 直接使用JsRuntime
fn test1(){
    let ext = Extension::builder()
    .ops(vec![
      all_dict::decl(),
    ])
    .build();
  // Initialize a runtime instance
  let mut runtime = JsRuntime::new(RuntimeOptions {
    extensions: vec![ext],
    startup_snapshot: Some(deno_isolate_init()),
    ..Default::default()
  });
    let code = r#"
    Deno.core.print("hello world");
   let value =  Deno.core.opSync('all_dict');
     for(let i = 0;i<value.length;i++){
        Deno.core.print(value[i].dict_name+"\n");
     }    
    "#;
    runtime.execute_script("script_name", code).unwrap();
}

//规则引擎测试类 默认使用全量的模式 MainWorker
fn test(){
    let start = Instant::now();
    let mut workers = init();
    info!("instance workers time {} 毫秒",start.elapsed().as_millis().to_string());
    let code = r#"
    console.log(this);
   let value =  Deno.core.opSync('all_dict');
     for(let i = 0;i<value.length;i++){
        console.log(value[i]);
     }    
    "#;
    workers.execute_script("script_name", code);
}
#[op]
fn all_dict() -> Result<Vec<SysDictTypeDTO>, deno_core::error::AnyError> {
    let vo = async_std::task::block_on(async { get_all_list().await });
    Ok(vo.unwrap())
}
fn get_error_class_name(e: &AnyError) -> &'static str {
    deno_runtime::errors::get_error_class_name(e).unwrap_or("Error")
}

pub fn init() -> MainWorker {
    let module_loader = Rc::new(FsModuleLoader);
    let create_web_worker_cb = Arc::new(|_| {
        todo!("Web workers are not supported in the example");
    });
    let web_worker_preload_module_cb = Arc::new(|_| {
        todo!("Web workers are not supported in the example");
    });
    let e = Extension::builder().ops(vec![all_dict::decl()]).build();
    let options = WorkerOptions {
        bootstrap: BootstrapOptions {
            args: vec![],
            apply_source_maps: false,
            cpu_count: 1,
            debug_flag: false,
            enable_testing_features: false,
            location: None,
            no_color: false,
            is_tty: false,
            runtime_version: "x".to_string(),
            ts_version: "x".to_string(),
            unstable: false,
        },
        extensions:vec![e],
        unsafely_ignore_certificate_errors: None,
        root_cert_store: None,
        user_agent: "cassie_engine".to_string(),
        seed: None,
        js_error_create_fn: None,
        web_worker_preload_module_cb,
        create_web_worker_cb,
        maybe_inspector_server: None,
        should_break_on_first_statement: false,
        module_loader,
        get_error_class_fn: Some(&get_error_class_name),
        origin_storage_dir: None,
        blob_store: BlobStore::default(),
        broadcast_channel: InMemoryBroadcastChannel::default(),
        shared_array_buffer_store: None,
        compiled_wasm_module_store: None,
    };

    let js_path = Path::new(env!("CARGO_MANIFEST_DIR"));
    let main_module = deno_core::resolve_path(&js_path.to_string_lossy()).unwrap();
    let permissions = Permissions::allow_all();
    MainWorker::bootstrap_from_options(main_module.clone(), permissions, options)
}
