use sqlparser::{ast::Expr, dialect::GenericDialect, parser::Parser};

fn parse() {
  let sql = "SELECT t3.* FROM sys_role_user AS t1 LEFT JOIN sys_role_menu AS t2 ON t1.role_id = t2.role_id LEFT JOIN sys_menu AS t3 ON t2.menu_id = t3.id WHERE t1.user_id = ? AND t3.del_flag = 0 AND t3.agency_code = ? AND t3.menu_type = ? ";
  let dialect = GenericDialect {}; // or AnsiDialect, or your own dialect ...
                                   //解析sql
  let atc = Parser::parse_sql(&dialect, &sql).unwrap();
  if let Some(data) = atc.get(0) {
    match data {
      sqlparser::ast::Statement::Query(q) => {
        if let sqlparser::ast::SetExpr::Select(select) = &q.body {
          if let Some(selection) = &select.selection {
            let ok = deep(&selection.clone());
            print!("{}", ok);
          }
        }
      }
      _ => {}
    }
  }
}
fn deep(selection: &Expr) -> bool {
  match selection {
    sqlparser::ast::Expr::Identifier(e) => {
      println!("进来了:{}", e.value.clone());
      if e.value == "agency_code" {
        return true;
      }
    }
    sqlparser::ast::Expr::BinaryOp { left, op: _, right } => {
      println!("进来了1:{:?}", *left.clone());
      if deep(&*left) || deep(&*right) {
        return true;
      }
    }
    sqlparser::ast::Expr::CompoundIdentifier(co) => {
      for c in co {
        if c.value == "agency_code" {
          return true;
        }
      }
    }
    _ => {}
  }
  return false;
}
