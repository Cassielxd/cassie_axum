use cassie_config::config::ApplicationConfig;
use sqlparser::ast::Assignment;
use sqlparser::ast::OnInsert;
use sqlparser::ast::Select;
use sqlparser::ast::SqliteOnConflict;
use sqlparser::ast::Statement::{Delete, Insert, Update};
use sqlparser::ast::TableWithJoins;
use sqlparser::ast::{
    BinaryOperator, Expr, Ident, ObjectName, Query, SetExpr, Value as Text, Values,
};

use crate::APPLICATION_CONTEXT; //构建租户化insert语句
///构建插入语句
pub fn build_insert(
    agency_code: String,
    or: Option<SqliteOnConflict>,
    table_name: ObjectName,
    columns: Vec<Ident>,
    overwrite: bool,
    source: Box<Query>,
    partitioned: Option<Vec<Expr>>,
    after_columns: Vec<Ident>,
    table: bool,
    on: Option<OnInsert>,
) -> Option<String> {
    let config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    let mut c = columns.clone();
    let agency_column = Ident {
        value: config.tenant().column().clone(),
        quote_style: None,
    };
    //判断columns是否为空已经包含了租户字段
    if !columns.contains(&agency_column) {
        c.push(agency_column);
        match &source.body {
            sqlparser::ast::SetExpr::Values(value) => {
                let mut values = value.clone().0;
                //组装value 如果是 insert into table values (1,2,3),(4,5,6)  多values 的情况就需要循环处理 把租户字段添加进去
                for elem in values.iter_mut() {
                    elem.push(sqlparser::ast::Expr::Value(Text::SingleQuotedString(
                        agency_code.clone(),
                    )));
                }
                let insert = Insert {
                    or: or.clone(),
                    table_name: table_name.clone(),
                    columns: c,
                    overwrite: overwrite.clone(),
                    source: Box::new(Query {
                        body: SetExpr::Values(Values(values)),
                        ..*source.clone()
                    }),
                    partitioned: partitioned.clone(),
                    after_columns: after_columns.clone(),
                    table: table.clone(),
                    on: on.clone(),
                };
                return Some(insert.to_string());
            }
            _ => return None,
        }
    }
    return None;
}
///构建租户化where条件语句
pub fn build_where(agency_code: String, selection: Option<Expr>) -> Option<Expr> {
    let config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    let u_selection = match selection {
        //如果原SQL已经有查询条件 则直接加上 租户化条件
        Some(selection) => Option::from(Expr::BinaryOp {
            left: Box::new(selection),
            op: BinaryOperator::And,
            right: Box::new(Expr::BinaryOp {
                left: Box::new(Expr::Identifier(Ident {
                    value: config.tenant().column().clone(),
                    quote_style: None,
                })),
                op: BinaryOperator::Eq,
                right: Box::new(Expr::Value(Text::SingleQuotedString(agency_code))),
            }),
        }),
        //如果原始sql里没有查询条件 则直接加上租户化条件
        None => Some(Expr::BinaryOp {
            left: Box::new(Expr::Identifier(Ident {
                value: config.tenant().column().clone(),
                quote_style: None,
            })),
            op: BinaryOperator::Eq, //这里是 =
            right: Box::new(Expr::Value(Text::SingleQuotedString(agency_code))),
        }),
    };
    u_selection
}
//构建更新语句
pub fn build_update(
    agency_code: String,
    table: TableWithJoins,
    assignments: Vec<Assignment>,
    selection: Option<Expr>,
) -> String {
    let update = Update {
        table: table,
        assignments: assignments,
        selection: build_where(agency_code, selection),
    };
    return update.to_string();
}
///构建删除语句
pub fn build_delete(
    agency_code: String,
    table_info: ObjectName,
    selection: Option<Expr>,
) -> String {
    let delete = Delete {
        table_name: table_info,
        selection: build_where(agency_code, selection),
    };
    return delete.to_string();
}

//构建租户化select
pub fn build_select(select: Select, agency_code: String) -> String {
    let config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    //克隆一份做备份
    let mut select1 = select.clone();
    //处理租户字段
    select1.selection = build_where(agency_code, select1.selection.clone());
    return select1.to_string();
}

//递归查找 租户列是否存在 存在则返回true
pub fn deep_find(selection: &Expr) -> bool {
    let config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    let column = config.tenant().column().clone();
    match selection {
        //判断具体的某个查询条件是不是租户字段
        sqlparser::ast::Expr::Identifier(e) => {
            if e.value == column {
                return true;
            }
        }
        // 多个查询条件递归处理最终会走到 其他的两个分支
        sqlparser::ast::Expr::BinaryOp { left, op, right } => {
            //递归
            if deep_find(&*left) || deep_find(&*right) {
                return true;
            }
        }
        //处理  t.agency_code = ? 的情况
        sqlparser::ast::Expr::CompoundIdentifier(co) => {
            for c in co {
                if c.value == column {
                    return true;
                }
            }
        }
        //忽略其他情况
        _ => {}
    }
    return false;
}

pub fn has_table(table_info: ObjectName, config: &ApplicationConfig) -> bool {
    let ObjectName(table_info) = table_info;
    let tableinfo = table_info.get(0).unwrap();
    //判断有表名称没有被忽略
    for table in config.tenant().ignore_table().iter() {
        if table.eq(&tableinfo.value) {
            return true;
        }
    }
    false
}
//判断查询语句是不是需要租户化
pub fn intercept_query(select: Select) -> bool {
    let from = select.from;
    //拿到where查询条件
    if let Some(selection) = select.selection {
        //如果租户列已经存在了就不需要再租户化了 查询条件里已经存在了
        if deep_find(&selection) {
            return false;
        }
    }
    let config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    //获取到 from 表名 有可能存在 join
    for elem in from.iter() {
        let relation = &elem.relation;
        match relation {
            sqlparser::ast::TableFactor::Table {
                name,
                alias,
                args,
                with_hints,
            } => {
                //获取到表名称
                if has_table(name.clone(), config) {
                    return false;
                }
            }
            _ => println!(),
        }
    }
    true
}

pub fn intercept_update(elem: TableWithJoins, selection: Option<Expr>) -> bool {
    //拿到where查询条件
    if let Some(selection) = selection {
        //如果租户列已经存在了就不需要再租户化了 查询条件里已经存在了
        if deep_find(&selection) {
            return false;
        }
    }
    let config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    //获取到 from 表名 有可能存在 join

    let relation = &elem.relation;
    match relation {
        sqlparser::ast::TableFactor::Table {
            name,
            alias,
            args,
            with_hints,
        } => {
            //获取到表名称
            if has_table(name.clone(), config) {
                return false;
            }
        }
        _ => println!(),
    }

    true
}

pub fn intercept_delete(table_info: ObjectName, selection: Option<Expr>) -> bool {
    //拿到where查询条件
    if let Some(selection) = selection {
        //如果租户列已经存在了就不需要再租户化了 查询条件里已经存在了
        if deep_find(&selection) {
            return false;
        }
    }
    let config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    //获取到 from 表名 有可能存在 join
    if has_table(table_info, config) {
        return false;
    }
    true
}
