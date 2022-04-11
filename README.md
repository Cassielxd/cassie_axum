# cassie_axum

#### 介绍
基于rust axum 完成web端手脚架 基础权限,用户,缓存,验证,代码生成

rust axum web 是rust开发的web手脚架项目
前端项目 https://gitee.com/stringlxd/cassie_admin

#### 项目讲解
B站：
https://space.bilibili.com/480402847?spm_id_from=333.788.b_765f7570696e666f.1
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

 1. 初始化DB，添加配置
   导入db/cassie_admin.sql文件
   配置application.yml
 2. 构建并启动项目
   cargo build
   cago run
 3. 启动前端项目https://gitee.com/stringlxd/cassie_admin
    默认端口 9999

#### 目录结构
```
cassie_axum
├─ axum.postman_collection.json //postmain测试api
├─ Cargo.lock
├─ Cargo.toml                   //核心包配置
├─ cassie_casbin_adapter        
│  ├─ Cargo.toml
│  └─ src
│     ├─ action.rs      //casbin操作定义
│     ├─ cici_adapter.rs//适配器
│     ├─ lib.rs
│     └─ models.rs      //casbin模型定义             
├─ cassie_common                
│  ├─ Cargo.toml
│  └─ src
│     ├─ error.rs
│     ├─ lib.rs
│     └─ utils
│        ├─ bencher.rs
│        ├─ mod.rs
│        ├─ password_encoder.rs //密码加密解密
│        └─ string.rs//字符串工具类
├─ cassie_config
│  ├─ Cargo.toml
│  └─ src
│     ├─ config.rs//核心配置类
│     └─ lib.rs
├─ cassie_domain
│  ├─ Cargo.toml
│  └─ src
│     ├─ dto
│     │  ├─ asi_dto.rs
│     │  ├─ mod.rs
│     │  ├─ sign_in.rs
│     │  ├─ sys_auth_dto.rs
│     │  ├─ sys_dict_dto.rs
│     │  ├─ sys_event_dto.rs
│     │  ├─ sys_log.rs
│     │  ├─ sys_menu_dto.rs
│     │  ├─ sys_params_dto.rs
│     │  ├─ sys_role_dto.rs
│     │  └─ sys_user_dto.rs
│     ├─ entity
│     │  ├─ asi_entitys.rs
│     │  ├─ event.rs
│     │  ├─ log.rs
│     │  ├─ mod.rs
│     │  ├─ pagedata.rs
│     │  ├─ sms.rs
│     │  └─ sys_entitys.rs
│     ├─ lib.rs
│     ├─ request
│     │  ├─ mod.rs
│     │  ├─ request_model.rs//thread_local 用户信息
│     │  └─ tree.rs //TreeService 树节点生成
│     └─ vo
│        ├─ jwt.rs
│        ├─ mod.rs
│        └─ sign_in.rs
├─ cassie_orm
│  ├─ Cargo.toml
│  └─ src
│     ├─ dao
│     │  ├─ mapper.rs//rbaits 核心实现
│     │  └─ mod.rs
│     ├─ lib.rs
│     └─ mapper
│        └─ menu_mapper.html//sql文件映射
├─ cassie_rules
│  ├─ Cargo.toml
│  └─ src
│     ├─ core
│     │  ├─ mod.rs
│     │  └─ rules.rs
│     ├─ lib.rs
│     └─ secript.rhai
├─ cassie_tests
│  ├─ Cargo.toml
│  └─ src
│     ├─ lib.rs
│     └─ main.rs
├─ cassie_upload
│  ├─ Cargo.toml
│  └─ src
│     ├─ lib.rs
│     └─ upload
│        ├─ mod.rs
│        ├─ oss_service.rs//oss上传核心实现
│        └─ upload_service.rs
├─ cassie_web
│  ├─ application.yml
│  ├─ auth_config
│  │  └─ rbac_with_domains_model.conf
│  ├─ Cargo.toml
│  └─ src
│     ├─ admin      //后台管理resource核心实现
│     │  ├─ asi    //动态表单相关
│     │  │  ├─ asi_group_column_resource.rs
│     │  │  ├─ asi_group_resource.rs
│     │  │  ├─ asi_group_values_resource.rs
│     │  │  └─ mod.rs
│     │  ├─ mod.rs
│     │  └─ sys     //系统相关
│     │     ├─ mod.rs
│     │     ├─ sys_auth_resource.rs 
│     │     ├─ sys_dict_type_resource.rs
│     │     ├─ sys_dict_value_resource.rs
│     │     ├─ sys_menu_resource.rs
│     │     ├─ sys_params_resource.rs
│     │     ├─ sys_role_resource.rs
│     │     ├─ sys_upload_resource.rs
│     │     └─ sys_user_resource.rs
│     ├─ api
│     │  └─ mod.rs
│     ├─ cici_casbin  //casbin 实现
│     │  ├─ casbin_service.rs
│     │  └─ mod.rs
│     ├─ config
│     │  ├─ log.rs
│     │  └─ mod.rs
│     ├─ initialize   //系统启动初始化相关
│     │  ├─ casbin.rs
│     │  ├─ config.rs
│     │  ├─ database.rs
│     │  ├─ event.rs
│     │  ├─ mod.rs
│     │  ├─ rules.rs
│     │  └─ service.rs
│     ├─ interceptor
│     │  ├─ interceptor.rs //租户拦截器
│     │  └─ mod.rs
│     ├─ lib.rs
│     ├─ main.rs
│     ├─ middleware      //中间件实现
│     │  ├─ auth.rs      //权限拦截实现
│     │  ├─ event.rs     //操作日志拦截器
│     │  └─ mod.rs
│     ├─ nacos
│     │  └─ mod.rs
│     ├─ observe         //event 核心实现
│     │  ├─ consumer.rs
│     │  ├─ event.rs
│     │  └─ mod.rs
│     ├─ routers  
│     │  ├─ admin.rs     //管理端路由
│     │  ├─ api.rs       //api路由
│     │  └─ mod.rs
│     └─ service         //service核心包
│        ├─ asi
│        │  ├─ asi_service.rs
│        │  ├─ asi_validation.rs
│        │  └─ mod.rs
│        ├─ event
│        │  ├─ event_service.rs
│        │  └─ mod.rs
│        ├─ log
│        │  ├─ log_service.rs
│        │  └─ mod.rs
│        ├─ mod.rs
│        └─ sys
│           ├─ cache_service.rs
│           ├─ crud_service.rs
│           ├─ mod.rs
│           ├─ redis_service.rs
│           ├─ sys_auth_service.rs
│           ├─ sys_dict_service.rs
│           ├─ sys_menu_service.rs
│           ├─ sys_params_service.rs
│           ├─ sys_role_data_scope_service.rs
│           ├─ sys_role_menu_service.rs
│           ├─ sys_role_service.rs
│           ├─ sys_role_user_service.rs
│           └─ sys_user_service.rs
├─ db
│  ├─ cassie_admin.sql    //基础sql脚本  
│  └─ cassie_admin_v1.sql
├─ LICENSE
└─ README.md

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


