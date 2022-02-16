# cassie_axum

#### 介绍
基于rust axum 完成web端手脚架 基础权限,用户,缓存,验证,代码生成

rust axum web 是rust开发的web手脚架项目

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

#### 已完成

1. 基础缓存定义,redis
2. orm框架选用Ribatis
3. casbin-rs集成,适配器编写
4. 用户权限jwt 融合casbin-rs

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
    // 初始化日志
    tracing_subscriber::fmt::init();
    //绑定端口 初始化 路由
    let app = Router::new()
        .route("/index", get(index)).layer(extractor_middleware::<Auth>())
        .nest("/admin", admin::routers())
        .nest("/api", api::routers());
    println!("address:{}", &CONTEXT.config.server);
    axum::Server::bind(&CONTEXT.config.server.parse().unwrap())
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


#### 更新日志

2022.2.15
```rust
  1:更新casbin初始化权限脚本
  2:添加 resource/:id 资源验证方式
  3:部分代码重构
```