#![allow(unused_variables)]
#![allow(unused_must_use)]
/**
*struct:CasbinRule
*desc:casbin_rule 框架权限表
*author:String
*email:348040933@qq.com
*/
#[crud_table(table_name:casbin_rule)]
#[derive(Clone, Debug)]
pub struct CasbinRule {
  pub id: u64,
  pub ptype: String,
  pub v0: String,
  pub v1: String,
  pub v2: String,
  pub v3: String,
  pub v4: String,
  pub v5: String,
}

impl_field_name_method!(CasbinRule {
  id,
  ptype,
  v0,
  v1,
  v2,
  v3,
  v4,
  v5,
});
#[derive(Debug, Default)]
pub struct NewCasbinRule<'a> {
  pub ptype: &'a str,
  pub v0: &'a str,
  pub v1: Option<&'a str>,
  pub v2: Option<&'a str>,
  pub v3: Option<&'a str>,
  pub v4: Option<&'a str>,
  pub v5: Option<&'a str>,
}
