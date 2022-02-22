use std::time::Duration;
use async_std::task;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use crate::CASSIE_CONFIG;
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'{').add(b'}').add(b':').add(b',');
pub fn register_service() {
    println!("register service: {:?}", CASSIE_CONFIG.nacos_server);

    task::spawn(
        async {
            let client = reqwest::blocking::Client::new();
            let body = client.post(
                format!("{}/v1/ns/instance?serviceName={}&ip={}&port={}",
                CASSIE_CONFIG.nacos_server,
                CASSIE_CONFIG.application_name,
                CASSIE_CONFIG.host,
                CASSIE_CONFIG.port).as_str()
            ).send().unwrap().text();
            println!("{:?}", body);
        }
    );
}

fn ping() {
    //
    // nacos 文档中没有说明 metadata 必选, 测试发现，如果没有 metadata 信息， java 端会有错误
    //
    let beat = format!("{{\"serviceName\":\"{}\",\"ip\":\"{}\",\"port\":\"{}\",\"weight\":1,\"metadata\":{{}}}}", CASSIE_CONFIG.application_name, CASSIE_CONFIG.host, CASSIE_CONFIG.port);
    let  encode = utf8_percent_encode(&beat, FRAGMENT).to_string();
    task::spawn(
        async move {

            let client = reqwest::blocking::Client::new();
            let _body = client.put(
                format!("{}/v1/ns/instance/beat?serviceName={}&beat={}",
                CASSIE_CONFIG.nacos_server,
                CASSIE_CONFIG.application_name,
                        encode
                ).as_str()
            ).send().unwrap().text();
            println!("ping result:{:?}", _body);
        }
    );
}

pub fn ping_schedule() {
    println!("ping schedule");
    loop {
        std::thread::sleep(Duration::from_secs(1));
        ping();
    }
}