# cassie_JsRuntime

#### 介绍
JavaScript脚本运行时 是基于 Deno Runtime 进行的二次封装
封装并添加了 cassie 系统函数 供JavaScript脚本运行时调用

####  核心代码
```rust
  //初始化 JsRuntime核心方法
  //args 参数可为None
 pub async fn init(args: Option<Vec<String>>) -> MainWorker {
    let start = Instant::now();
    //Module脚本加载器 FsModuleLoader
    let module_loader = Rc::new(FsModuleLoader);
    let options = WorkerOptions {
        //启动参数
        bootstrap: get_option(args),
        //初始化系统操作 ops  
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
    //加载全局js 并初始化 
    let js_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("js/cassie.js");
    let main_module = deno_core::resolve_path(&js_path.to_string_lossy()).unwrap();
    info!("{}", main_module.clone());
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
        //cpu数量
        cpu_count: std::thread::available_parallelism().map(|p| p.get()).unwrap_or(1),
        //是否开启debugger
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
```
#### 使用说明

 ```rust
#[tokio::main]
async fn main() {
    let mut workers = init(None).await;
    //let config = Cassie.getConfig();获取系统配置
    //let all_dict = Cassie.getAllDict();获取系统所有字典项
    //let user_info = Cassie.getUserById("1");根据id取用户信息
    let code =r#"
      //获取当前请求 request_context
      let user_info = Cassie.getUserById("1");
     console.log(user_info);
    "#;
    workers.execute_script("script_name",code).unwarp();
}
     
 ```

#### 项目中如何使用
用户系统异步事件处理
```rust
   //事件消费处理类
pub async fn init_consumer() {
    let pharos = APPLICATION_CONTEXT.get::<SharedPharos<CassieEvent>>();
    let mut events = pharos.observe_shared(Channel::Unbounded.into()).await.unwrap();
    let mut workers = init(None).await;
    loop {
        let event = events.next().await.unwrap();
        consume(&mut workers, event).await;
    }
}
//事件消费 待二次开发
pub async fn consume(worker: &mut MainWorker, e: CassieEvent) {
    //在这里是获取不到 thread_local 的值 异步消费过来 已经不在同一个线程里了
    match e {
        //登录事件
        CassieEvent::LogLogin(dto) => {
            let mut entity = dto.into();
            let log_login_service = APPLICATION_CONTEXT.get::<LogLoginService>();
            log_login_service.save(&mut entity).await;
        }
        //操作事件
        CassieEvent::LogOperation(dto) => {
            let mut entity = dto.into();
            let log_operation_service = APPLICATION_CONTEXT.get::<LogOperationService>();
            log_operation_service.save(&mut entity).await;
        }
        //消息事件
        CassieEvent::Sms { sms_type } => todo!("待开发"),
        //自定义事件
        CassieEvent::Custom(custom) => {
            let event_config_service = APPLICATION_CONTEXT.get::<EventConfigService>();
            //获取到所有的事件配置
            let list = load_event().await;
            if let Ok(data) = list {
                let d = data
                    .iter()
                    .filter(|item| key_match2(&custom.path.clone().as_str(), &item.path().clone().unwrap()) || item.path().clone().unwrap().contains(&custom.path.clone()))
                    .collect::<Vec<_>>();
                info!("事件个数：{:?}", d.len());
                if d.len() > 0 {
                    execute_script(worker, d, &custom).await;
                }
            }
        }
    }
}

//核心动态脚本执行方法
async fn execute_script(workers: &mut MainWorker, data: Vec<&EventConfigDTO>, custom: &CustomEvent) {
    let start = Instant::now();
    let init_code = format!(
        r#" var request_context=JSON.parse({});"#,
        serde_json::to_string_pretty(&serde_json::to_string_pretty(&json!(custom)).unwrap()).unwrap()
    );
    for event in data {
        let code = build_script(init_code.clone(), event.event_script().clone().unwrap().clone());
        match workers.js_runtime.execute_script(event.event_name().clone().unwrap().as_str(), code.as_str()) {
            Ok(data) => {}
            Err(e) => {
                info!("error info {:#?}", e.to_string());
            }
        }
    }
    workers.run_event_loop(false).await;
    info!("execute script time {} 毫秒", start.elapsed().as_millis().to_string());
}
//构建脚本每个脚本独立运行上下文隔离
fn build_script(init_code: String, event_script: String) -> String {
    let script = format!(
        r#"
        "use strict";
        ((window) => {{
         {}
         {}
        }})(this);
    "#,
        init_code, event_script
    );
    script
}

//发布事件
pub async fn fire_event(e: CassieEvent) {
    let pharos = APPLICATION_CONTEXT.get::<SharedPharos<CassieEvent>>();
    pharos.notify(e).await;
}

```
调用链
```
  fire_event---提交异步事件给->pharos 
  init_consumer----pharos异步消费事件>consume
  当事件类型是CassieEvent::Custom自定义事件时 执行JavaScript脚本
  execute_script
  
```



