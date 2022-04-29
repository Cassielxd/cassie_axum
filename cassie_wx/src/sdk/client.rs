use bytes::Bytes;
use cassie_common::error::Error;
use cassie_common::error::Result;

use reqwest::{header, Client as HttpClient};
use serde::Serialize;
use std::time::Duration;
const DEFAULT_USER_AGENT: &'static str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/70.0.3534.4 Safari/537.36";
pub struct Client {
  pub(crate) client: HttpClient,
}
impl Client {
  pub fn new() -> Self {
    let mut headers = header::HeaderMap::new();
    headers.insert(header::USER_AGENT, header::HeaderValue::from_static(DEFAULT_USER_AGENT));

    Client {
      client: HttpClient::builder()
        .timeout(Duration::from_secs(300))
        .connect_timeout(Duration::from_secs(300))
        .default_headers(headers)
        .build()
        .unwrap(),
    }
  }
  pub async fn post<T: Serialize + ?Sized>(&self, url: &str, params: &T) -> Result<String> {
    match self.client.post(url).json(params).send().await {
      Ok(res) => {
        if res.status() == 200 {
          match res.text().await {
            Ok(txt) => {
              // println!("--- {} ----", txt);
              Ok(txt)
            }
            Err(e) => Err(Error::E(e.to_string())),
          }
        } else {
          Err(Error::E(format!("status={}", res.status())))
        }
      }
      Err(e) => Err(Error::E(format!("Send request error: {}", e))),
    }
  }

  pub async fn post_betyes(&self, url: &str, body: Bytes) -> Result<String> {
    match self.client.post(url).body(body).send().await {
      Ok(res) => {
        if res.status() == 200 {
          match res.text().await {
            Ok(txt) => {
              // println!("--- {} ----", txt);
              Ok(txt)
            }
            Err(e) => Err(Error::E(e.to_string())),
          }
        } else {
          Err(Error::E(format!("status={}", res.status())))
        }
      }
      Err(e) => Err(Error::E(format!("Send request error: {}", e))),
    }
  }
  #[allow(dead_code)]
  pub async fn get(&self, url: &str) -> Result<String> {
    match self.client.get(url).send().await {
      Ok(res) => {
        if res.status() == 200 {
          match res.text().await {
            Ok(txt) => {
              // println!("--- {} ----", txt);
              Ok(txt)
            }
            Err(e) => Err(Error::E(e.to_string())),
          }
        } else {
          Err(Error::E(format!("status={}", res.status())))
        }
      }
      Err(e) => Err(Error::E(format!("Send request error: {}", e))),
    }
  }
}
