pub mod mp;
pub mod sdk;
pub mod wxapp;
pub mod pay;
use cassie_common::error::Error;
use cassie_common::error::Result;
///
#[inline]
pub fn json_decode(data: &str) -> Result<serde_json::Value> {
  let obj: serde_json::Value = match serde_json::from_str(data) {
    Ok(decoded) => decoded,
    Err(ref e) => {
      return Err(Error::E(format!("Json decode error: {}", e)));
    }
  };
  let dic = obj.as_object().unwrap();
  let code = if dic.contains_key("errcode") {
    "errcode"
  } else {
    "code"
  };

  let code = match obj[code].as_i64() {
    Some(v) => v,
    None => 0,
  };
  if code != 0 {
    let msg: String = if dic.contains_key("msg") {
      obj["msg"].to_string()
    } else {
      obj["errmsg"].to_string()
    };
    return Err(Error::E(msg));
  }
  println!("obj====={:?}", obj);
  Ok(obj)
}

/// 获取当前时间戮
pub fn current_timestamp() -> u64 {
  use std::time::{SystemTime, UNIX_EPOCH};
  SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs() as u64
}
