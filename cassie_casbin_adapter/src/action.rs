#![allow(unused_must_use)]
use casbin::{Filter, Result};
use rbatis::{
    crud::{CRUDMut, Skip, CRUD},
    rbatis::Rbatis,
};

use crate::models::{CasbinRule, NewCasbinRule};

/**
 *method:filtered_where_values
 *desc:获取筛选参数
 *author:String
 *email:348040933@qq.com
 */
fn filtered_where_values<'a>(filter: &Filter<'a>) -> ([&'a str; 6], [&'a str; 6]) {
    let mut g_filter: [&'a str; 6] = ["%", "%", "%", "%", "%", "%"];
    let mut p_filter: [&'a str; 6] = ["%", "%", "%", "%", "%", "%"];
    for (idx, val) in filter.g.iter().enumerate() {
        if val != &"" {
            g_filter[idx] = val;
        }
    }
    for (idx, val) in filter.p.iter().enumerate() {
        if val != &"" {
            p_filter[idx] = val;
        }
    }
    (g_filter, p_filter)
}

fn normalize_casbin_rule(mut rule: Vec<String>, field_index: usize) -> Vec<String> {
    rule.resize(6 - field_index, String::from(""));
    rule
}

/**
 *struct:CICICasinService
 *desc:casbin权限CRUD服务
 *author:String
 *email:348040933@qq.com
 */
pub struct CICICasinService {
    pub rbatis: Rbatis,
}

impl CICICasinService {
    /**
     *method:load_policy
     *desc:加载策略
     *author:String
     *email:348040933@qq.com
     */
    pub async fn load_policy(&self) -> Result<Vec<CasbinRule>> {
        let list = self
            .rbatis
            .fetch_list_by_wrapper::<CasbinRule>(self.rbatis.new_wrapper())
            .await
            .map_err(|e| casbin::error::Error::IoError(e.into()));
        list
    }
    pub(crate) async fn load_filtered_policy<'a>(
        &self,
        filter_x: &Filter<'_>,
    ) -> Result<Vec<CasbinRule>> {
        let (g_filter, p_filter) = filtered_where_values(filter_x);
        let mut result = Vec::new();
        let gwrapper = self
            .rbatis
            .new_wrapper()
            .like(CasbinRule::ptype(), "p%")
            .eq(CasbinRule::v0(), g_filter[0])
            .eq(CasbinRule::v1(), g_filter[1])
            .eq(CasbinRule::v2(), g_filter[2])
            .eq(CasbinRule::v3(), g_filter[3])
            .eq(CasbinRule::v4(), g_filter[4])
            .eq(CasbinRule::v5(), g_filter[5]);

        let glist = self
            .rbatis
            .fetch_list_by_wrapper::<CasbinRule>(gwrapper)
            .await;
        if let Ok(l) = glist {
            for p in l {
                result.push(p);
            }
        }
        let pwrapper = self
            .rbatis
            .new_wrapper()
            .like(CasbinRule::ptype(), "g%")
            .eq(CasbinRule::v0(), p_filter[0])
            .eq(CasbinRule::v1(), p_filter[1])
            .eq(CasbinRule::v2(), p_filter[2])
            .eq(CasbinRule::v3(), p_filter[3])
            .eq(CasbinRule::v4(), p_filter[4])
            .eq(CasbinRule::v5(), p_filter[5]);
        let plist = self
            .rbatis
            .fetch_list_by_wrapper::<CasbinRule>(pwrapper)
            .await;
        if let Ok(l) = plist {
            for p in l {
                result.push(p);
            }
        }

        Ok(result)
    }
    /**
     *method:save_policy
     *desc:保存策略
     *author:String
     *email:348040933@qq.com
     */
    pub async fn save_policy(&self, rules: Vec<NewCasbinRule<'_>>) -> Result<()> {
        let mut tx = self.rbatis.acquire_begin().await.unwrap();
        for rule in rules {
            let v1 = rule.v1.unwrap_or("");
            let v2 = rule.v2.unwrap_or("");
            let v3 = rule.v3.unwrap_or("");
            let v4 = rule.v4.unwrap_or("");
            let v5 = rule.v5.unwrap_or("");
            let gwrapper = self
                .rbatis
                .new_wrapper()
                .eq(CasbinRule::ptype(), rule.ptype)
                .eq(CasbinRule::v0(), rule.v0)
                .eq(CasbinRule::v1(), v1)
                .eq(CasbinRule::v2(), v2)
                .eq(CasbinRule::v3(), v3)
                .eq(CasbinRule::v4(), v4)
                .eq(CasbinRule::v5(), v5);
            let s = tx.fetch_by_wrapper::<CasbinRule>(gwrapper).await;
            if s.is_ok() {
                continue;
            }
            let e = CasbinRule {
                id: 0,
                ptype: rule.ptype.to_string(),
                v0: rule.v0.to_string(),
                v1: v1.to_string(),
                v2: v2.to_string(),
                v3: v3.to_string(),
                v4: v4.to_string(),
                v5: v5.to_string(),
            };
            tx.save::<CasbinRule>(&e, &[Skip::Column(CasbinRule::id())])
                .await;
        }
        tx.commit().await.unwrap();
        println!("保存策略!!!!!!!!!!!!!!!");
        Ok(())
    }
    /**
     *method:clear_policy
     *desc:清除策略
     *author:String
     *email:348040933@qq.com
     */
    pub(crate) async fn clear_policy(&self) -> Result<()> {
        self.rbatis
            .remove_by_wrapper::<CasbinRule>(self.rbatis.new_wrapper())
            .await;
        Ok(())
    }
    /**
     *method:add_policy
     *desc:添加策略
     *author:String
     *email:348040933@qq.com
     */
    pub(crate) async fn add_policy(&self, rule: NewCasbinRule<'_>) -> Result<bool> {
        let mut re = Vec::new();
        re.push(rule);
        self.save_policy(re).await;
        Ok(true)
    }

    pub async fn remove_policy(&self, pt: &str, rule: Vec<String>) -> Result<bool> {
        let rule = normalize_casbin_rule(rule, 0);
        let mut wa = self.rbatis.new_wrapper().eq(CasbinRule::ptype(), pt);
        if !rule[0].is_empty() {
            wa = wa.eq(CasbinRule::v0(), &rule[0].clone());
        }
        if !rule[1].is_empty() {
            wa = wa.eq(CasbinRule::v1(), &rule[1].clone());
        }
        if !rule[2].is_empty() {
            wa = wa.eq(CasbinRule::v2(), &rule[2].clone());
        }
        if !rule[3].is_empty() {
            wa = wa.eq(CasbinRule::v3(), &rule[3].clone());
        }
        if !rule[4].is_empty() {
            wa = wa.eq(CasbinRule::v4(), &rule[4].clone());
        }
        if !rule[5].is_empty() {
            wa = wa.eq(CasbinRule::v5(), &rule[5].clone());
        }
        self.rbatis.remove_by_wrapper::<CasbinRule>(wa).await;
        Ok(true)
    }
    pub async fn remove_policies(&self, pt: &str, rules: Vec<Vec<String>>) -> Result<bool> {
        for rule in rules {
            self.remove_policy(pt, rule);
        }
        Ok(true)
    }
    ///  删除筛选的策略
    pub async fn remove_filtered_policy(
        &self,
        pt: &str,
        field_index: usize,
        field_values: Vec<String>,
    ) -> Result<bool> {
        let field_values_x = normalize_casbin_rule(field_values, field_index);
        let mut wrapper = self.rbatis.new_wrapper().eq(CasbinRule::ptype(), pt);
        match field_index {
            1 => {
                if field_values_x[0].is_empty() {
                    wrapper = wrapper.eq(CasbinRule::v1(), field_values_x[0].clone());
                }
                if field_values_x[1].is_empty() {
                    wrapper = wrapper.eq(CasbinRule::v2(), field_values_x[1].clone());
                }
                if field_values_x[2].is_empty() {
                    wrapper = wrapper.eq(CasbinRule::v3(), field_values_x[2].clone());
                }
                if field_values_x[3].is_empty() {
                    wrapper = wrapper.eq(CasbinRule::v4(), field_values_x[3].clone());
                }
                if field_values_x[4].is_empty() {
                    wrapper = wrapper.eq(CasbinRule::v5(), field_values_x[4].clone());
                }
            }
            2 => {
                if field_values_x[0].is_empty() {
                    wrapper = wrapper.eq(CasbinRule::v2(), field_values_x[0].clone());
                }
                if field_values_x[1].is_empty() {
                    wrapper = wrapper.eq(CasbinRule::v3(), field_values_x[1].clone());
                }
                if field_values_x[2].is_empty() {
                    wrapper = wrapper.eq(CasbinRule::v4(), field_values_x[2].clone());
                }
                if field_values_x[3].is_empty() {
                    wrapper = wrapper.eq(CasbinRule::v5(), field_values_x[3].clone());
                }
            }
            3 => {
                if field_values_x[0].is_empty() {
                    wrapper = wrapper.eq(CasbinRule::v3(), field_values_x[0].clone());
                }
                if field_values_x[1].is_empty() {
                    wrapper = wrapper.eq(CasbinRule::v4(), field_values_x[1].clone());
                }
                if field_values_x[2].is_empty() {
                    wrapper = wrapper.eq(CasbinRule::v5(), field_values_x[2].clone());
                }
            }
            4 => {
                if field_values_x[0].is_empty() {
                    wrapper = wrapper.eq(CasbinRule::v4(), field_values_x[0].clone());
                }
                if field_values_x[1].is_empty() {
                    wrapper = wrapper.eq(CasbinRule::v5(), field_values_x[1].clone());
                }
            }
            5 => {
                if field_values_x[0].is_empty() {
                    wrapper = wrapper.eq(CasbinRule::v5(), field_values_x[0].clone());
                }
            }
            _ => {
                if field_values_x[0].is_empty() {
                    wrapper = wrapper.eq(CasbinRule::v0(), field_values_x[0].clone());
                }
                if field_values_x[1].is_empty() {
                    wrapper = wrapper.eq(CasbinRule::v1(), field_values_x[1].clone());
                }
                if field_values_x[2].is_empty() {
                    wrapper = wrapper.eq(CasbinRule::v2(), field_values_x[2].clone());
                }
                if field_values_x[3].is_empty() {
                    wrapper = wrapper.eq(CasbinRule::v3(), field_values_x[3].clone());
                }
                if field_values_x[4].is_empty() {
                    wrapper = wrapper.eq(CasbinRule::v4(), field_values_x[4].clone());
                }
                if field_values_x[5].is_empty() {
                    wrapper = wrapper.eq(CasbinRule::v5(), field_values_x[5].clone());
                }
            }
        }
        self.rbatis.remove_by_wrapper::<CasbinRule>(wrapper).await;
        Ok(true)
    }
}
