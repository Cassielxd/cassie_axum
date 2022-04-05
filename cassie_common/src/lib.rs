pub mod error;
pub mod utils;

use axum::{body::Body, response::Response};
use error::Error;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
pub const CODE_SUCCESS: i8 = 0;
pub const CODE_FAIL: i8 = -1;

/// http接口返回模型结构，提供基础的 code，msg，data 等json数据结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RespVO<T> {
    pub code: Option<i8>,
    pub msg: Option<String>,
    pub data: Option<T>,
}

impl<T> RespVO<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    pub fn from_result(arg: &Result<T, Error>) -> Self {
        if arg.is_ok() {
            Self {
                code: Some(CODE_SUCCESS),
                msg: None,
                data: arg.clone().ok(),
            }
        } else {
            Self {
                code: Some(CODE_FAIL),
                msg: Some(arg.clone().err().unwrap().to_string()),
                data: None,
            }
        }
    }

    pub fn from(arg: &T) -> Self {
        Self {
            code: Some(CODE_SUCCESS),
            msg: None,
            data: Some(arg.clone()),
        }
    }

    pub fn from_error(arg: &Error) -> Self {
        Self {
            code: Some(CODE_FAIL),
            msg: Some(arg.to_string()),
            data: None,
        }
    }

    pub fn from_error_info(code: i8, info: &str) -> Self {
        Self {
            code: Some(code),
            msg: Some(info.to_string()),
            data: None,
        }
    }

    pub fn resp_json(&self) -> Response<Body> {
        Response::builder()
            .extension(|| {})
            .header("Access-Control-Allow-Origin", "*")
            .header("Cache-Control", "no-cache")
            .header("Content-Type", "text/json;charset=UTF-8")
            .body(Body::from(self.to_string()))
            .unwrap()
    }
}

impl<T> ToString for RespVO<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
