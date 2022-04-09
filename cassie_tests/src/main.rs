use sqlparser::{ast::Expr, dialect::GenericDialect, parser::Parser};

fn main() {
    let sql = "select id,pid,url,name,menu_type,icon,permissions,sort,del_flag,creator,create_date,updater,update_date,method,path from sys_menu
    where agency_code = ? AND menu_type = ?";
    let dialect = GenericDialect {}; // or AnsiDialect, or your own dialect ...
                                     //解析sql
    let atc = Parser::parse_sql(&dialect, &sql).unwrap();

    if let Some(data) = atc.get(0) {
        match data {
            sqlparser::ast::Statement::Query(q) => {
                if let sqlparser::ast::SetExpr::Select(select) = &q.body {
                    if let Some(selection) = &select.selection {
                        let ok = deep(selection.clone());
                        print!("{}", ok);
                    }
                }
            }
            _ => {}
        }
    }
}

fn deep(selection: Expr) -> bool {
    match selection {
        sqlparser::ast::Expr::Identifier(e) => {
            println!("进来了:{}", e.value.clone());
            if e.value == "agency_code" {
                return true;
            }
        }
        sqlparser::ast::Expr::BinaryOp { left, op, right } => {
            println!("进来了1:{:?}", *left.clone());
            if deep(*left) || deep(*right) {
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
