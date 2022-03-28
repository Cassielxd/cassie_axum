use log::info;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use tokio::time;

use crate::{CONTAINER, config::config::ApplicationConfig};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'{').add(b'}').add(b':').add(b',');

//nacos服务注册
pub async fn register_service() {
    let CASSIE_CONFIG= CONTAINER.get::<ApplicationConfig>();
    if CASSIE_CONFIG.nacos.nacos_flag {
        info!("register service: {:?}", CASSIE_CONFIG.nacos.nacos_server);
        let client = reqwest::Client::new();
        let body = client.post(
            format!("{}/v1/ns/instance?serviceName={}&ip={}&port={}",
                    CASSIE_CONFIG.nacos.nacos_server,
                    CASSIE_CONFIG.nacos.application_name,
                    CASSIE_CONFIG.server.host,
                    CASSIE_CONFIG.server.port).as_str()
        ).send().await.unwrap().text().await;
        match body {
            Ok(s) => {
                info!("nacos连接成功");
                tokio::task::spawn(ping_schedule());
            }
            Err(e) => info!("nacos连接失败"),
        }
    }
}

//nacos心跳检测
pub async fn ping() {
    let CASSIE_CONFIG= CONTAINER.get::<ApplicationConfig>();
    //
    // nacos 文档中没有说明 metadata 必选, 测试发现，如果没有 metadata 信息， java 端会有错误
    //
    let beat = format!("{{\"serviceName\":\"{}\",\"ip\":\"{}\",\"port\":\"{}\",\"weight\":1,\"metadata\":{{}}}}", CASSIE_CONFIG.nacos.application_name, CASSIE_CONFIG.server.host, CASSIE_CONFIG.server.port);
    let encode = utf8_percent_encode(&beat, FRAGMENT).to_string();
    let client = reqwest::Client::new();
    client.put(
        format!("{}/v1/ns/instance/beat?serviceName={}&beat={}",
                CASSIE_CONFIG.nacos.nacos_server,
                CASSIE_CONFIG.nacos.application_name,
                encode
        ).as_str()
    ).send().await;
}

pub async fn ping_schedule() {
    info!("nacos心跳检测开始");
    let mut interval = time::interval(time::Duration::from_secs(10));
    loop {
        interval.tick().await;
        ping().await;
    }
}
