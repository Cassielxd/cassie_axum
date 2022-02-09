#![allow(clippy::type_complexity)]

use std::sync::Arc;

use casbin::{prelude::{TryIntoAdapter, TryIntoModel}, DefaultModel, MgmtApi};
use cassie_common::error::{Error, Result};

use casbin::{CachedEnforcer, CoreApi, Result as CasbinResult};


use crate::casbin_adapter::{cici_adapter::CICIAdapter};
use async_std::sync::RwLock;
use crate::cici_casbin::cici_match;


#[derive(Clone)]
pub struct CasbinVals {
    pub subject: String,
    pub domain: Option<String>,
}

/**
 *struct:CasbinService
 *desc:casbin 权限处理核心service
 *author:String
 *email:348040933@qq.com
 */
#[derive(Clone)]
pub struct CasbinService {
    pub enforcer: Arc<RwLock<CachedEnforcer>>,
}

impl CasbinService {
    /**
            sign_in
             *method:default
             *desc:初始化casbin 上下文 加载model 初始化 CICIAdapter casbin适配器
             *author:String
             *email:348040933@qq.com
     */
    pub fn default() -> Self {
        let cached_enforcer = async_std::task::block_on(async {
            let m = DefaultModel::from_file("cassie_web/auth_config/rbac_with_domains_model.conf")
                .await
                .unwrap();
            let a = CICIAdapter::new();

            let mut cached_enforcer = CachedEnforcer::new(m, a).await.unwrap();
            cached_enforcer.add_function("ciciMatch", cici_match);
            cached_enforcer
        });
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
    /**
         *method:call
         *desc:核心验证方法 path ,action ,vals
         *author:String
         *email:348040933@qq.com
     */
    pub async fn call(&mut self, path: String, action: String, vals: CasbinVals) -> bool {
        /*获取验证器*/
        let cloned_enforcer = self.enforcer.clone();
        let subject = vals.subject.clone();
        /*获取对应的 用户 用户为空直接返回false*/
        if !vals.subject.is_empty() {
            /*判断是否是多租户模型*/
            if let Some(domain) = vals.domain {
                let mut lock = cloned_enforcer.write().await;
                match lock.enforce_mut(vec![subject, domain, path, action]) {
                    Ok(true) => true,
                    Ok(false) => false,
                    Err(e) => false
                }
            } else {
                let mut lock = cloned_enforcer.write().await;
                match lock.enforce_mut(vec![subject, path, action]) {
                    Ok(true) => true,
                    Ok(false) => false,
                    Err(_) => false
                }
            }
        } else {
            false
        }
    }
}

