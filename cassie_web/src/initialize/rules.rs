use crate::service::ops::init_sys_ops;
use deno_core::JsRuntime;
use deno_core::RuntimeOptions;
use deno_runtime::deno_broadcast_channel::InMemoryBroadcastChannel;
use deno_runtime::deno_core::error::AnyError;
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
pub async  fn init_rules() {
    test().await
}

fn get_workers()->Arc<Mutex<MainWorker>>{
    static mut MAIN_WORKER: Option<Arc<Mutex<MainWorker>>> = None;
    unsafe {// Rust中使用可变静态变量都是unsafe的
        MAIN_WORKER.get_or_insert_with(|| {
            // 初始化单例对象的代码
            Arc::new(Mutex::new(init()))
        }).clone()
    }
}
//默认使用 最小化的模式 基础 deno_core 直接使用JsRuntime
fn test1(){

  // Initialize a runtime instance
  let mut runtime = JsRuntime::new(RuntimeOptions {
    extensions: vec![init_sys_ops()],
    startup_snapshot: Some(deno_isolate_init()),
    ..Default::default()
  });
    let code = r#"
    Deno.core.print("hello world");
   let value =  Deno.core.opSync('op_all_dict');
     for(let i = 0;i<value.length;i++){
        Deno.core.print(value[i].dict_name+"\n");
     }    
    "#;
    runtime.execute_script("script_name", code).unwrap();
    
}

//规则引擎测试类 默认使用全量的模式 包含http localstorage 等 MainWorker
async fn test(){
    let start = Instant::now();
    let mut workers = init();
    
    info!("instance workers time {} 毫秒",start.elapsed().as_millis().to_string());
    let code = r#"
    let value =  Deno.core.opSync('op_config');
        console.log(value);
    "#;
    workers.execute_script("script_name", code);
    workers.run_event_loop(false).await;
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
    
    let options = WorkerOptions {
        bootstrap: get_test_option(),
        extensions:vec![init_sys_ops()],
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

fn get_test_option()->BootstrapOptions{
    BootstrapOptions {
        args: vec!["cassie".to_string().to_string()],
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
    }
}