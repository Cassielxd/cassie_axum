use crate::json_decode;
use crate::sdk::client::Client;
use crate::sdk::constant::API_DOMAIN;

use cassie_common::error::Error;
use cassie_common::error::Result;
/// 获取session_key
/// GET https://api.weixin.qq.com/sns/jscode2session?appid=APPID&secret=SECRET&js_code=JSCODE&grant_type=authorization_code
/// 登录凭证校验。通过 wx.login 接口获得临时登录凭证 code 后传到开发者服务器调用此接口完成登录流程。
/// DOC: https://developers.weixin.qq.com/miniprogram/dev/api-backend/open-api/login/auth.code2Session.html
/// @param1: appid	string		是	小程序 appId
/// @param2: secret	string		是	小程序 appSecret
/// @param3: js_code	string		是	登录时获取的 code
/// @param4: 无需传入,grant_type	string		是	授权类型，此处只需填写 authorization_code
pub async fn get_session_key(
  appid: &str,
  secret: &str,
  code: &str,
) -> Result<serde_json::Value> {
  let url = format!("{api}/sns/jscode2session?appid={appid}&secret={secret}&js_code={code}&grant_type=authorization_code",
        api= API_DOMAIN,
        appid=appid,
        code=code,
        secret=secret
    );
  let api = Client::new();
  let res = api.get(&url).await?;
  match json_decode(&res) {
    Ok(data) => {
      println!("{:?}", data.clone());
      if data.get("errcode").is_some() {
        Err(Error::E(format!(
          "auth error: {}",
          data["errmsg"].as_str().unwrap_or("")
        )))
      } else {
        Ok(data)
      }
    }
    Err(err) => {
      return Err(err);
    }
  }
}
