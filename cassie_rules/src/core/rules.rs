use rhai::{Engine, RegisterNativeFunction, Scope, AST};
use std::collections::HashMap;

pub struct RulesContext {
    pub engine: Engine,
    pub ast_map: HashMap<String, AST>,
}
impl Default for RulesContext {
    fn default() -> Self {
        Self {
            engine: Default::default(),
            ast_map: HashMap::new(),
        }
    }
}
impl RulesContext {
    ///加载所有规则并编译 失败则不加入缓存
    pub fn load_rules(&mut self, rule: HashMap<String, String>) {
        for (key, value) in rule.into_iter() {
            let ast = self.engine.compile(value);
            match ast {
                Ok(exp) => {
                    self.ast_map.insert(key, exp);
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
    }
    ///向引擎中添加一个方法
    pub fn add_function<A, F>(&mut self, name: &str, func: F)
    where
        F: RegisterNativeFunction<A, ()>,
    {
        self.engine.register_fn(name, func);
    }
    ///直接执行脚本字符串并拿到返回值
    pub fn run(&self, script: &str) {
        let res = self.engine.run(script);
        match res {
            Ok(_) => println!("执行成功"),
            Err(e) => println!("{}", e),
        }
    }
    ///根据脚本id 执行AST
    pub fn run_ast(&self, id: &str) {
        let ast = self.ast_map.get(id);
        match ast {
            Some(ast) => {
                self.engine.run_ast(ast);
            }
            None => {
                println!("{}", "没有找到规则");
            }
        }
    }
    ///根据脚本id执行AST
    pub fn run_ast_with_scope(&self, scope: &mut Scope, id: &str) {
        let ast = self.ast_map.get(id);
        match ast {
            Some(ast) => {
                self.engine.run_ast_with_scope(scope, ast);
            }
            None => {
                println!("{}", "没有找到规则");
            }
        }
    }
}
