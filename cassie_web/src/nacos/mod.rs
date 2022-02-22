use async_std::task;
use log::info;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use tokio::time;
use crate::CASSIE_CONFIG;
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'{').add(b'}').add(b':').add(b',');
pub async fn register_service() {
    info!("register service: {:?}", CASSIE_CONFIG.nacos_server);
     let client = reqwest::Client::new();
     let body = client.post(
        format!("{}/v1/ns/instance?serviceName={}&ip={}&port={}",
        CASSIE_CONFIG.nacos_server,
        CASSIE_CONFIG.application_name,
        CASSIE_CONFIG.host,
        CASSIE_CONFIG.port).as_str()
    ).send().await.unwrap();

}

pub async fn ping() {
    //
    // nacos 文档中没有说明 metadata 必选, 测试发现，如果没有 metadata 信息， java 端会有错误
    //
    let beat = format!("{{\"serviceName\":\"{}\",\"ip\":\"{}\",\"port\":\"{}\",\"weight\":1,\"metadata\":{{}}}}", CASSIE_CONFIG.application_name, CASSIE_CONFIG.host, CASSIE_CONFIG.port);
    let  encode = utf8_percent_encode(&beat, FRAGMENT).to_string();
    let client = reqwest::Client::new();
    let _body = client.put(
        format!("{}/v1/ns/instance/beat?serviceName={}&beat={}",
        CASSIE_CONFIG.nacos_server,
        CASSIE_CONFIG.application_name,
                encode
        ).as_str()
    ).send().await.unwrap();
  
}

pub async fn ping_schedule() {
    info!("nacos心跳检测开始");
    let mut interval = time::interval(time::Duration::from_secs(10));
    loop {
        interval.tick().await;
        ping().await;
    }
}
