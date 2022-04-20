use crate::service::ops::init_sys_ops;
use crate::APPLICATION_CONTEXT;
use cassie_config::config::ApplicationConfig;
use deno_runtime::deno_broadcast_channel::InMemoryBroadcastChannel;
use deno_runtime::deno_core::error::AnyError;
use deno_runtime::deno_core::FsModuleLoader;
use deno_runtime::deno_web::BlobStore;
use deno_runtime::permissions::Permissions;
use deno_runtime::worker::MainWorker;
use deno_runtime::worker::WorkerOptions;
use deno_runtime::{deno_core, BootstrapOptions};
use log::info;
use serde_json::json;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use tokio::time::Instant;
//初始话规则引擎
pub async fn init_rules() {
    //规则引擎测试使用 test().await;
}

//规则引擎测试类 默认使用全量的模式 包含http localstorage 等 MainWorker
async fn test() {
    let start = Instant::now();
    let mut workers = init(None);
    info!(
        "instance workers time {} 毫秒",
        start.elapsed().as_millis().to_string()
    );
    let config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    let payload = json!(config);
    let init = format!(
        r#" var request_context=JSON.parse({});
       console.log(request_context.redis_url);
    "#,
        serde_json::to_string_pretty(&serde_json::to_string_pretty(&payload).unwrap()).unwrap()
    );
    workers.execute_script("script_name", &init);
    let code = r#" console.log(request_context);"#;
    workers.execute_script("script_name", code);
    workers.run_event_loop(false).await;
}

fn get_error_class_name(e: &AnyError) -> &'static str {
    deno_runtime::errors::get_error_class_name(e).unwrap_or("Error")
}

pub fn init(args: Option<Vec<String>>) -> MainWorker {
    let module_loader = Rc::new(FsModuleLoader);
    let create_web_worker_cb = Arc::new(|_| {
        todo!("Web workers are not supported in the example");
    });
    let web_worker_preload_module_cb = Arc::new(|_| {
        todo!("Web workers are not supported in the example");
    });

    let options = WorkerOptions {
        bootstrap: get_option(args),
        extensions: vec![init_sys_ops()],
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

fn get_option(args: Option<Vec<String>>) -> BootstrapOptions {
    let arg = match args {
        Some(a) => a,
        None => vec![],
    };
    BootstrapOptions {
        args: arg,
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
