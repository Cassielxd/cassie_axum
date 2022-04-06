# cassie_axum

#### 介绍
基于rust axum 完成web端手脚架 基础权限,用户,缓存,验证,代码生成

rust axum web 是rust开发的web手脚架项目
前端项目 https://gitee.com/stringlxd/cassie_admin

#### 软件架构

软件架构说明

1. rust web框架
2. web:axum
3. 数据库:mysql
4. Orm:Ribatis
5. RBAC:cabin-rs
6. 验证码:captcha
7. 数据验证:validator

#### 开发计划

1. 基础缓存定义,ORM框架定义
2. 返回数据格式定义
3. 完成casbin的基础RABC权限集成
4. TODO 微服务集成 KONG网关集成
5. nacos注册中心集成
6. 动态表单实现
7. 微信小程序工具集开发
8. 租户化实现

#### 已完成

1. 基础缓存定义,redis
2. orm框架选用Ribatis
3. casbin-rs集成,适配器编写
4. 用户权限jwt 融合casbin-rs
5. 完成nacos注册和心跳集成
6. 动态表单实现
7. 租户化实现

#### 使用说明

```rust 
 rust开发环境省略
 1:初始化DB，添加配置
   导入db cassie_admin.sql
 2:构建并启动项目
   cargo build
   cago run
 3:postman 导入 axum.postman_collection.json
   开始测试
```

```rust
/*
  
  权限中间件使用:
  Auth是权限认证验证的核心入口
  中间件的添加方式：
  route("/index", get(index)).layer(extractor_middleware::<Auth>())
  访问/index的时候受权限影响
*/

#[tokio::main]
async fn main() {
    //初始化上环境下文
    init_context().await;
    let cassie_config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    init_log();
    info!(
        " - Local:   http://{}:{}",
        cassie_config.server.host.replace("0.0.0.0", "127.0.0.1"),
        cassie_config.server.port
    );
    //nacos 服务注册
    register_service().await;
    let server = format!(
        "{}:{}",
        cassie_config.server.host, cassie_config.server.port
    );

    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers(Any)
        .max_age(Duration::from_secs(60) * 10);

    //绑定端口 初始化 路由
    let app = Router::new()
        .route("/", get(index))
        .nest(
            "/admin",
            admin::routers().layer(extractor_middleware::<Auth>()),
        )
        .nest("/api", api::routers())
        .layer(cors);
    // 启动服务
    Server::bind(&server.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

#### 框架说明
管理员账号:admin/123456
测试账号:lixingdong1/123456
1. https://casbin.org/docs/zh-CN/overview
2. https://rbatis.github.io/rbatis.io/#/
3. https://axum.rs/

#### 参与贡献

String <348040933@qq.com>
交流群:435604279

感谢Ribatis作者

#### 提示
代码每天都在更新,大家每天及时更新
#### 更新日志
2022.4.6
````````````````````````````````````````````````````````````````
//添加登录日志功能
//添加操作日志功能
 //构建操作日志event对象
            let event = CassieEvent::LogOperation {
                operation: Some(action.clone()),
                request_uri: Some(path.clone()),
                ip: None,
                creator_name,
                request_params: None,
                request_method: Some(action.clone()),
                request_time: Some(start.elapsed().as_millis().to_string()),
                status,
            };
            //发布事件
            fire_event(event).await;
````````````````````````````````````````````````````````````````

2022.4.5
````````````````````````````````````````````````````````````````
//完成eventBus 1.0开发
//事件基本对象
pub enum CassieEvent {
    Log {},//日志
    Sms {
        sms_type: u8,//消息
    },
    Custom {
        event_type: u8, //自定义事件 1 脚本 2 其他业务分类(待定)
        data: HashMap<String, String>,
    },
}
//发布事件
 let pharos = APPLICATION_CONTEXT.get::<SharedPharos<CassieEvent>>();
        pharos.notify(CassieEvent::Log {}).await;

````````````````````````````````````````````````````````````````



2022.3.31
````````````````````````````````````````````````````````````````
完成租户化开发
配置 application.yml
tenant:
  enable: true   //开启租户
  column: "agency_code" //租户字段
  ignore_table:   //忽略表
    - "sys_log_login"
````````````````````````````````````````````````````````````````



2022.3.29
````````````````````````````````````````````````````````````````
包结构重新梳理
替换缓存和tread_loacal实现方式
````````````````````````````````````````````````````````````````

2022.3.18
````````````````````````````````````````````````````````````````
完成动态表单功能
mongodb 配置 application.yml
```yml
mongodb_url: "mongodb://localhost:27017"
```
1. 业务分组定义 
2. colums定义 mysql存储
3. value使用 mongodb存储
4. 表单类型分为 from 和table
5. entity_id 是业务id
````````````````````````````````````````````````````````````````
2022.2.22
``````````````````````````````````````````````````````````````````````
完成nacos注册和心跳
nacos 配置 application.yml
```yml
nacos_server: "http://127.0.0.1:8848/nacos"
application_name: "cassie_admin"
```
``````````````````````````````````````````````````````````````````````````````
2022.2.15
```rust
  1:更新casbin初始化权限脚本
  2:添加 resource/:id 资源验证方式
  3:部分代码重构
```
