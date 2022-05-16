use deno_core::{error::AnyError, FsModuleLoader};
use deno_runtime::{
    deno_broadcast_channel::InMemoryBroadcastChannel,
    deno_web::BlobStore,
    permissions::Permissions,
    worker::{MainWorker, WorkerOptions},
    BootstrapOptions,
};
use gag::{BufferRedirect, Gag};
use std::rc::Rc;
use std::sync::Arc;
use std::{io::Read, path::Path};
#[tokio::main]
async fn main() {
    let mut print_gag = Gag::stdout().unwrap();
    println!("No one will see this!");
    if true {
        drop(print_gag);
        println!("They will see this...");
        print_gag = Gag::stdout().unwrap();
        println!("Not this...");
    }
    println!("Nor this.");

    /* let mut buf = BufferRedirect::stdout().unwrap();
    let mut worker = getmanWorker();
    worker.execute_script("code", "console.log('hello world')").unwrap();
    let mut output = String::new();
    buf.read_to_string(&mut output).unwrap();
    drop(buf);
    println!("console:{}", &output[..]);*/
}
fn get_error_class_name(e: &AnyError) -> &'static str {
    deno_runtime::errors::get_error_class_name(e).unwrap_or("Error")
}
fn getmanWorker() -> MainWorker {
    let module_loader = Rc::new(FsModuleLoader);
    let create_web_worker_cb = Arc::new(|_| {
        todo!("Web workers are not supported in the example");
    });
    let web_worker_preload_module_cb = Arc::new(|_| {
        todo!("Web workers are not supported in the example");
    });

    let options = WorkerOptions {
        bootstrap: BootstrapOptions {
            args: vec![],
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
        extensions: vec![],
        unsafely_ignore_certificate_errors: None,
        root_cert_store: None,
        user_agent: "hello_runtime".to_string(),
        seed: None,
        source_map_getter: None,
        format_js_error_fn: None,
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
        stdio: Default::default(),
    };

    let js_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/hello_runtime.js");
    let main_module = deno_core::resolve_path(&js_path.to_string_lossy()).unwrap();
    let permissions = Permissions::allow_all();

    let mut worker = MainWorker::bootstrap_from_options(main_module.clone(), permissions, options);
    worker
}
