use cassie_domain::vo::jwt::JWTToken;
pub mod auth_admin;
pub mod auth_api;
pub mod event;
use cassie_common::error::Error;
use cassie_config::config::ApplicationConfig;
use cassie_domain::request::RequestModel;
use std::sync::{Arc, Mutex};

use crate::APPLICATION_CONTEXT;
/**
 *method:checked_token
 *desc:校验token是否有效，未过期
 *author:String
 *email:348040933@qq.com
 */
pub async fn checked_token(token: &str) -> Result<JWTToken, Error> {
  //check token alive
  let cassie_config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
  let token = JWTToken::verify(cassie_config.jwt_secret(), token);
  token
}

pub fn get_local() -> Option<RequestModel> {
  let request_model =
    APPLICATION_CONTEXT.try_get_local::<Arc<Mutex<RequestModel>>>();
  match request_model {
    None => None,
    Some(e) => {
      let e = e.lock().unwrap();
      let mut model = RequestModel::default();
      model.set_uid(e.uid().clone());
      model.set_agency_code(e.agency_code().clone());
      model.set_super_admin(e.super_admin().clone());
      model.set_username(e.username().clone());
      model.set_path(e.path().clone());
      model.set_from(e.from().clone());
      Some(model)
    }
  }
}

pub fn set_local(data: JWTToken, path: String) {
  let request_model =
    APPLICATION_CONTEXT.try_get_local::<Arc<Mutex<RequestModel>>>();
  match request_model {
    Some(d) => {
      let a = d.clone();
      let mut model = a.lock().unwrap();
      model.set_uid(data.id().clone());
      model.set_agency_code(data.agency_code().clone());
      model.set_super_admin(data.super_admin().clone());
      model.set_username(data.username().clone());
      model.set_path(path);
      model.set_from(data.from().clone());
    }
    None => {
      APPLICATION_CONTEXT.set_local(move || {
        let mut model = RequestModel::default();
        model.set_uid(data.id().clone());
        model.set_agency_code(data.agency_code().clone());
        model.set_super_admin(data.super_admin().clone());
        model.set_username(data.username().clone());
        model.set_path(path.clone());
        model.set_from(data.from().clone());
        let mutex = Arc::new(Mutex::new(model));
        return mutex;
      });
    }
  }
}
