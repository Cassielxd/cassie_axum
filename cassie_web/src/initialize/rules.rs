use crate::service::ops::init_sys_ops;
use deno_runtime::deno_broadcast_channel::InMemoryBroadcastChannel;
use deno_runtime::deno_core::error::AnyError;
use deno_runtime::deno_core::FsModuleLoader;
use deno_runtime::deno_web::BlobStore;
use deno_runtime::permissions::Permissions;
use deno_runtime::worker::MainWorker;
use deno_runtime::worker::WorkerOptions;
use deno_runtime::{deno_core, BootstrapOptions};
use log::info;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use tokio::time::Instant;

//初始话规则引擎
pub async fn init_rules() {
    //规则引擎测试使用 test().await;
}

fn get_error_class_name(e: &AnyError) -> &'static str {
    deno_runtime::errors::get_error_class_name(e).unwrap_or("Error")
}

pub async fn init(args: Option<Vec<String>>) -> MainWorker {
    let start = Instant::now();
    let module_loader = Rc::new(FsModuleLoader);
    let options = WorkerOptions {
        bootstrap: get_option(args),
        extensions: vec![init_sys_ops()],
        unsafely_ignore_certificate_errors: None,
        root_cert_store: None,
        user_agent: "cassie_engine".to_string(),
        seed: None,
        web_worker_preload_module_cb: Arc::new(|_| unreachable!()),
        create_web_worker_cb: Arc::new(|_| unreachable!()),
        maybe_inspector_server: None,
        should_break_on_first_statement: false,
        module_loader,
        get_error_class_fn: Some(&get_error_class_name),
        origin_storage_dir: None,
        blob_store: BlobStore::default(),
        broadcast_channel: InMemoryBroadcastChannel::default(),
        shared_array_buffer_store: None,
        compiled_wasm_module_store: None,
        source_map_getter: None,
        format_js_error_fn: None,
        stdio: Default::default(),
    };

    let js_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("js/cassie.js");
    let main_module = deno_core::resolve_path(&js_path.to_string_lossy()).unwrap();
    info!("cassie js path {} ", main_module);
    let permissions = Permissions::allow_all();
    let mut main_worker = MainWorker::bootstrap_from_options(main_module.clone(), permissions, options);
    main_worker.execute_main_module(&main_module).await.unwrap();
    main_worker.run_event_loop(false).await.unwrap();
    info!("init JsRuntime time {} millisecond!", start.elapsed().as_millis().to_string());
    main_worker
}

fn get_option(args: Option<Vec<String>>) -> BootstrapOptions {
    let arg = match args {
        Some(a) => a,
        None => vec![],
    };
    BootstrapOptions {
        args: arg,
        cpu_count: std::thread::available_parallelism().map(|p| p.get()).unwrap_or(1),
        debug_flag: false,
        enable_testing_features: false,
        location: None,
        no_color: false,
        is_tty: false,
        runtime_version: "0.1".to_string(),
        ts_version: "x".to_string(),
        unstable: false,
    }
}
