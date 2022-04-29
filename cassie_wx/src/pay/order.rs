use crate::sdk::client::Client;
use crate::sdk::config::Config;
use cassie_common::error::Error;
use cassie_common::error::Result;
use chrono::prelude::Utc;
use itertools::Itertools;
use serde_json::json;
pub struct Order {
    conf: Config,
}

fn gen_sign(params: &serde_json::Value, secret_key: &str) -> String {
    let mut vs = vec![];
    if let Some(map) = params.as_object() {
        for key in map.keys().sorted() {
            vs.push(format!("{}={}", key, map[key].to_string().trim_matches(&['"', '"'] as &[_])));
        }
        vs.push(format!("{}={}", "key", secret_key));
        let wait_md5_str = format!("{}", vs.join("&"));
        let sign = format!("{:x}", md5::compute(&wait_md5_str)).to_uppercase();
        return sign;
    }

    format!("")
}

impl Order {
    pub fn new(conf: Config) -> Self {
        Order { conf: conf }
    }
    // 统一下单
    pub async fn create(&self, mut params: serde_json::Value) -> Result<serde_json::Value> {
        let api_url = "https://api.mch.weixin.qq.com/pay/unifiedorder";
        let conf = self.conf.clone();

        params["appid"] = json!(conf.app_id);
        params["mch_id"] = json!(conf.mch_id);
        //api
        let secret_key = conf.secret_key;

        params["sign"] = json!(gen_sign(&params, &secret_key));

        let body = format!("<xml>{}</xml>", serde_xml_rs::to_string(&params).unwrap_or_default());

        let request = Client::new();
        let r = request.post(&api_url, &body).await.unwrap_or_default();

        let ps = serde_xml_rs::from_str::<serde_json::Value>(&r).unwrap_or_default();
        // 交易类型
        let trade_type = params["trade_type"].as_str();

        if let Some(root_doms) = ps.as_object() {
            let rr: serde_json::Map<String, serde_json::Value> = root_doms.iter().map(|(i, vo)| (i.clone(), vo["$value"].clone())).collect();

            let prepayid = rr.get("prepay_id");

            if trade_type == Some("APP") && prepayid.is_some() {
                //二次签名

                let mut new_params = serde_json::Map::new();

                let timestamp = Utc::now().naive_local().timestamp();
                new_params.insert(format!("appid"), rr.get("appid").unwrap().clone());
                new_params.insert(format!("partnerid"), rr.get("mch_id").unwrap().clone());
                new_params.insert(format!("prepayid"), rr.get("prepay_id").unwrap().clone());
                new_params.insert(format!("package"), json!("Sign=WXPay"));
                new_params.insert(format!("noncestr"), rr.get("nonce_str").unwrap().clone());
                new_params.insert(format!("timestamp"), json!(timestamp));
                new_params.insert(format!("sign"), json!(gen_sign(&json!(new_params), &secret_key)));

                return Ok(serde_json::Value::Object(new_params));
            } else if trade_type == Some("JSAPI") && prepayid.is_some() {
                //二次签名
                let prepay_id = rr.get("prepay_id").unwrap().as_str().unwrap();
                let mut new_params = serde_json::Map::new();

                let timestamp = Utc::now().naive_local().timestamp();
                new_params.insert(format!("appId"), rr.get("appid").unwrap().clone());
                new_params.insert(format!("timeStamp"), json!(format!("{}", timestamp)));
                new_params.insert(format!("nonceStr"), rr.get("nonce_str").unwrap().clone());
                new_params.insert(format!("package"), json!(format!("prepay_id={}", prepay_id)));
                new_params.insert(format!("signType"), json!("MD5"));
                new_params.insert(format!("paySign"), json!(gen_sign(&json!(new_params), &secret_key)));

                return Ok(json!(new_params));
            }

            return Ok(json!(rr));
        }

        Ok(json!({
            "return_code": "FAIL",
            "return_msg": "请求网络问题"
        }))
    }
}
