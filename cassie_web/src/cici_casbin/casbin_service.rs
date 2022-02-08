#![allow(clippy::type_complexity)]

use std::sync::Arc;

use casbin::{prelude::{TryIntoAdapter, TryIntoModel}, DefaultModel, MgmtApi};
use cassie_common::error::{Error, Result};

use casbin::{CachedEnforcer, CoreApi, Result as CasbinResult};


use crate::casbin_adapter::{cici_adapter::CICIAdapter, cici_match};
use async_std::sync::RwLock;


#[derive(Clone)]
pub struct CasbinVals {
    pub subject: String,
    pub domain: Option<String>,
}

#[derive(Clone)]
pub struct CasbinService {
    pub enforcer: Arc<RwLock<CachedEnforcer>>,
}

impl CasbinService {
    pub async fn default() -> Self {
        let m = DefaultModel::from_file("cassie_web/auth_config/rbac_with_domains_model.conf")
            .await
            .unwrap();
        let a = CICIAdapter::new();

        let mut cached_enforcer = CachedEnforcer::new(m, a).await.unwrap();
        cached_enforcer.add_function("ciciMatch", cici_match);
        Self {
            enforcer: Arc::new(RwLock::new(cached_enforcer)),
        }
    }

    pub async fn new<M: TryIntoModel, A: TryIntoAdapter>(m: M, a: A) -> CasbinResult<Self> {
        let enforcer: CachedEnforcer = CachedEnforcer::new(m, a).await?;
        Ok(CasbinService {
            enforcer: Arc::new(RwLock::new(enforcer)),
        })
    }

    pub fn get_enforcer(&mut self) -> Arc<RwLock<CachedEnforcer>> {
        self.enforcer.clone()
    }

    pub fn set_enforcer(e: Arc<RwLock<CachedEnforcer>>) -> CasbinService {
        CasbinService { enforcer: e }
    }
    pub async fn call(&mut self, path: String, action: String, vals: CasbinVals) -> Result<bool> {
        let cloned_enforcer = self.enforcer.clone();
        let subject = vals.subject.clone();
        if !vals.subject.is_empty() {
            if let Some(domain) = vals.domain {
                let mut lock = cloned_enforcer.write().await;
                match lock.enforce_mut(vec![subject, domain, path, action]) {
                    Ok(true) => {
                        Ok(true)
                    }
                    Ok(false) => {
                        println!("验证失败");
                        Ok(false)
                    }
                    Err(e) => {
                        println!("验证异常{}", e.to_string());
                        Ok(false)
                    }
                }
            } else {
                let mut lock = cloned_enforcer.write().await;
                match lock.enforce_mut(vec![subject, path, action]) {
                    Ok(true) => {
                        Ok(true)
                    }
                    Ok(false) => {
                        Ok(false)
                    }
                    Err(_) => {
                        Ok(false)
                    }
                }
            }
        } else {
            Ok(false)
        }
    }
}

