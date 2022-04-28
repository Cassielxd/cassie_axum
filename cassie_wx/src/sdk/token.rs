use super::client::Client;
use super::constant::API_DOMAIN;
use cached::proc_macro::cached;
use cassie_common::error::Error;
use cassie_common::error::Result;

//获取token
#[cached(time = 7200, result = true, size = 10)]
pub async fn get_access_token(
  app_id: String,
  secret: String,
  grant_type: String,
) -> Result<String> {
  // 组装请求地址
  let url = format!(
        "{domain}/cgi-bin/token?grant_type={grant_type}&appid={app_id}&secret={secret}",
        domain = API_DOMAIN,
        grant_type = if grant_type.is_empty() {
            "client_credential".to_string()
        } else {
            grant_type
        },
        app_id = app_id,
        secret = secret
    );
  // 调用远程接口
  match Client::new().get(&url).await {
    Ok(res) => {
      match crate::json_decode(&res) {
        Ok(data) => {
          let token = match data["access_token"].as_str() {
            Some(s) => s.to_owned(),
            None => return Err(Error::E(format!("access token error"))),
          };

          // 将Token返出去
          return Ok(token);
        }
        Err(err) => {
          return Err(err);
        }
      }
    }
    Err(err) => Err(Error::E(format!("error {}", err))),
  }
}
