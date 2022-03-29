#![allow(unused_variables)]
#![allow(unused_must_use)]
use async_trait::async_trait;
use casbin::{Adapter, Filter, Model, Result};
use rbatis::rbatis::Rbatis;

use crate::{
    action::CICICasinService,
    models::{CasbinRule, NewCasbinRule},
};

/**
 *struct:CICIAdapter
 *desc:持久化权限适配器 casbin
 *author:String
 *email:348040933@qq.com
 */

pub struct CICIAdapter {
    service: CICICasinService,
    is_filtered: bool,
}

impl CICIAdapter {
    pub fn new(r: Rbatis) -> CICIAdapter {
        CICIAdapter {
            is_filtered: false,
            service: CICICasinService { rbatis: r },
        }
    }
    pub(crate) fn save_policy_line<'a>(
        &self,
        ptype: &'a str,
        rule: &'a [String],
    ) -> Option<NewCasbinRule<'a>> {
        if ptype.trim().is_empty() || rule.is_empty() {
            return None;
        }

        let mut new_rule = NewCasbinRule {
            ptype,
            v0: "",
            v1: Some(""),
            v2: Some(""),
            v3: Some(""),
            v4: Some(""),
            v5: Some(""),
        };

        new_rule.v0 = &rule[0];

        if rule.len() > 1 {
            new_rule.v1 = Some(&rule[1]);
        }

        if rule.len() > 2 {
            new_rule.v2 = Some(&rule[2]);
        }

        if rule.len() > 3 {
            new_rule.v3 = Some(&rule[3]);
        }

        if rule.len() > 4 {
            new_rule.v4 = Some(&rule[4]);
        }

        if rule.len() > 5 {
            new_rule.v5 = Some(&rule[5]);
        }
        Some(new_rule)
    }
    pub(crate) fn load_policy_line(&self, casbin_rule: &CasbinRule) -> Option<Vec<String>> {
        if casbin_rule.ptype.chars().next().is_some() {
            return self.normalize_policy(casbin_rule);
        }
        None
    }
    fn normalize_policy(&self, casbin_rule: &CasbinRule) -> Option<Vec<String>> {
        let mut result = vec![
            &casbin_rule.v0,
            &casbin_rule.v1,
            &casbin_rule.v2,
            &casbin_rule.v3,
            &casbin_rule.v4,
            &casbin_rule.v5,
        ];

        while let Some(last) = result.last() {
            if last.is_empty() {
                result.pop();
            } else {
                break;
            }
        }

        if !result.is_empty() {
            return Some(result.iter().map(|&x| x.to_owned()).collect());
        }
        None
    }
}

#[async_trait]
impl Adapter for CICIAdapter {
    async fn load_policy(&self, m: &mut dyn Model) -> Result<()> {
        let rules = self.service.load_policy().await?;
        for casbin_rule in &rules {
            let rule = self.load_policy_line(casbin_rule);
            if let Some(ref sec) = casbin_rule.ptype.chars().next().map(|x| x.to_string()) {
                if let Some(t1) = m.get_mut_model().get_mut(sec) {
                    if let Some(t2) = t1.get_mut(&casbin_rule.ptype) {
                        if let Some(rule) = rule {
                            t2.get_mut_policy().insert(rule);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    async fn load_filtered_policy<'a>(&mut self, m: &mut dyn Model, f: Filter<'a>) -> Result<()> {
        let rules = self.service.load_filtered_policy(&f).await?;
        self.is_filtered = true;
        for casbin_rule in &rules {
            if let Some(policy) = self.normalize_policy(casbin_rule) {
                if let Some(ref sec) = casbin_rule.ptype.chars().next().map(|x| x.to_string()) {
                    if let Some(t1) = m.get_mut_model().get_mut(sec) {
                        if let Some(t2) = t1.get_mut(&casbin_rule.ptype) {
                            t2.get_mut_policy().insert(policy);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    async fn save_policy(&mut self, m: &mut dyn Model) -> Result<()> {
        let mut rules = vec![];

        if let Some(ast_map) = m.get_model().get("p") {
            for (ptype, ast) in ast_map {
                let new_rules = ast
                    .get_policy()
                    .into_iter()
                    .filter_map(|x| self.save_policy_line(ptype, x));

                rules.extend(new_rules);
            }
        }

        if let Some(ast_map) = m.get_model().get("g") {
            for (ptype, ast) in ast_map {
                let new_rules = ast
                    .get_policy()
                    .into_iter()
                    .filter_map(|x| self.save_policy_line(ptype, x));

                rules.extend(new_rules);
            }
        }
        self.service.save_policy(rules).await
    }

    async fn clear_policy(&mut self) -> Result<()> {
        self.service.clear_policy().await
    }

    fn is_filtered(&self) -> bool {
        self.is_filtered
    }

    async fn add_policy(&mut self, sec: &str, ptype: &str, rule: Vec<String>) -> Result<bool> {
        if let Some(new_rule) = self.save_policy_line(ptype, rule.as_slice()) {
            return self.service.add_policy(new_rule).await;
        }
        Ok(false)
    }

    async fn add_policies(
        &mut self,
        sec: &str,
        ptype: &str,
        rules: Vec<Vec<String>>,
    ) -> Result<bool> {
        let new_rules = rules
            .iter()
            .filter_map(|x| self.save_policy_line(ptype, x))
            .collect::<Vec<NewCasbinRule>>();

        self.service.save_policy(new_rules).await;
        Ok(true)
    }

    async fn remove_policy(&mut self, sec: &str, ptype: &str, rule: Vec<String>) -> Result<bool> {
        self.service.remove_policy(ptype, rule).await
    }

    async fn remove_policies(
        &mut self,
        sec: &str,
        ptype: &str,
        rules: Vec<Vec<String>>,
    ) -> Result<bool> {
        self.service.remove_policies(ptype, rules).await
    }

    async fn remove_filtered_policy(
        &mut self,
        sec: &str,
        ptype: &str,
        field_index: usize,
        field_values: Vec<String>,
    ) -> Result<bool> {
        if field_index <= 5 && !field_values.is_empty() && field_values.len() >= 6 - field_index {
            Ok(false)
        } else {
            self.service
                .remove_filtered_policy(ptype, field_index, field_values)
                .await
        }
    }
}
