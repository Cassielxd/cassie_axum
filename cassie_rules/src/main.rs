use std::collections::HashMap;

use cassie_rules::core::rules::RulesContext;
use rhai::{Scope, Dynamic};

pub fn main() {
    let mut rc = RulesContext::default();
    let mut rule = HashMap::new();
    rule.insert("test".to_string(), "print(\"hello world\");".to_string());
    rc.load_rules(rule);
    rc.run_ast("test");
    let s =Scope::new();
}
