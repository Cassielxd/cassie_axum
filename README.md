# cassie_axum

#### 介绍
基于rust axum 完成web端手脚架 基础权限,用户,缓存,验证,代码生成

rust axum web 是rust开发的web手脚架项目
前端项目 https://gitee.com/stringlxd/cassie_admin

#### 项目讲解
B站：
https://space.bilibili.com/480402847?spm_id_from=333.788.b_765f7570696e666f.1
#### 项目文档
文档：https://www.yuque.com/heapw/hsr49u/kglmy5
#### 演示地址
http://47.104.64.212/#/login

#### 软件架构

软件架构说明

1. rust web框架
2. web:axum
3. 数据库:mysql
4. Orm:Ribatis
5. RBAC:cabin-rs
6. 验证码:captcha
7. 数据验证:validator
8. websocket:tokio-tungstenite

#### 开发计划

1. 基础缓存定义,ORM框架定义
2. 返回数据格式定义
3. 完成casbin的基础RABC权限集成
4. TODO 微服务集成 KONG网关集成
5. nacos注册中心集成
6. 动态表单实现
7. 微信小程序工具集开发
8. 租户化实现
9. javascript动态脚本集成

#### 已完成

1. 基础缓存定义,redis
2. orm框架选用Ribatis
3. casbin-rs集成,适配器编写
4. 用户权限jwt 融合casbin-rs
5. 完成nacos注册和心跳集成
6. 动态表单实现
7. 租户化实现

#### 使用说明
 前置 安装rust开发环境 遵循官网 
 如果开启了websocket 请放开20003端口
 否则连接不到
 1. 初始化DB，添加配置
   导入db/cassie_admin.sql文件
   配置application.yml
 2. 构建并启动项目
   cargo build
   cago run
 3. 启动前端项目https://gitee.com/stringlxd/cassie_admin
    默认端口 9999
 4. 打包 cargo  build --package cassie_web --relaese




#### 框架说明
管理员账号:admin/123456
测试账号:lixingdong1/123456
1. https://casbin.org/docs/zh-CN/overview
2. https://rbatis.github.io/rbatis.io/#/
3. https://axum.rs/
4. https://github.com/denoland/rusty_v8
5. https://github.com/denoland/deno

#### 参与贡献

String <348040933@qq.com>
交流群:435604279

感谢Ribatis作者

