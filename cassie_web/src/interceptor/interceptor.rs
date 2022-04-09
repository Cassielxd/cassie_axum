use crate::APPLICATION_CONTEXT;
use cached::proc_macro::cached;
use cassie_config::config::ApplicationConfig;
use cassie_domain::request::RequestModel;
use rbatis::plugin::intercept::SqlIntercept;
use rbatis::rbatis::Rbatis;
use rbatis::Error;
use rbson::Bson;
use sqlparser::ast::OnInsert;
use sqlparser::ast::Select;
use sqlparser::ast::SqliteOnConflict;
use sqlparser::ast::Statement::Insert;
use sqlparser::ast::Statement::Query as Iquwey;
use sqlparser::ast::{
    BinaryOperator, Expr, Ident, ObjectName, Query, SetExpr, Value as Text, Values,
};
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
//租户化拦截器 租户化核心实现
#[derive(Debug)]
pub struct AgencyInterceptor {
    //是否开始租户
    pub enable: bool,
    //租户的字段
    pub column: String,
    //需要忽略的表
    pub ignore_table: Vec<String>,
}
impl AgencyInterceptor {}
impl SqlIntercept for AgencyInterceptor {
    //sql拦截逻辑
    fn do_intercept(
        &self,
        rb: &Rbatis,
        sql: &mut String,
        args: &mut Vec<Bson>,
        is_prepared_sql: bool,
    ) -> Result<(), Error> {
        //判断是否开启租户化
        if self.enable {
            if let Some(request_model) = APPLICATION_CONTEXT.try_get_local::<RequestModel>() {
                *sql = build(sql.clone(), request_model.agency_code.clone());
            }
        }
        return Ok(());
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}
//判断查询语句是不是需要租户化
fn intercept_query(select: Select) -> bool {
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
                let ObjectName(table_info) = name.clone();
                let tableinfo = table_info.get(0).unwrap();
                //判断有表名称没有被忽略
                for table in config.tenant.ignore_table.iter() {
                    if table.eq(&tableinfo.value) {
                        return false;
                    }
                }
            }
            _ => println!(),
        }
    }
    true
}
//判断insert语句是不是需要租户化
fn intercept_insert(table_info: ObjectName) -> bool {
    let config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    let ObjectName(table_info) = table_info;
    let tableinfo = table_info.get(0).unwrap();

    for table in config.tenant.ignore_table.iter() {
        if table.eq(&tableinfo.value) {
            return false;
        }
    }
    true
}

//租户化sql生成
#[cached(time = 3600, size = 100)]
pub fn build(up_sql: String, agency_code: String) -> String {
    //获取配置类
    let config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    //获取默认数据库方言
    let dialect = GenericDialect {}; // or AnsiDialect, or your own dialect ...
                                     //解析sql
    let atc = Parser::parse_sql(&dialect, &up_sql).unwrap();
    //这里只有一条sql所以获取第一条  SQL可以解析多个
    if let Some(data) = atc.get(0) {
        //模式匹配 只管 select 的情况
        match data {
            Iquwey(q) => {
                if let sqlparser::ast::SetExpr::Select(select) = &q.body {
                    if intercept_query(*select.clone()) {
                        return build_select(*select.clone(), agency_code);
                    }
                }
            }
            Insert {
                or,
                table_name,
                columns,
                overwrite,
                source,
                partitioned,
                after_columns,
                table,
                on,
            } => {
                //判断是否需要租户化
                if intercept_insert(table_name.clone()) {
                    match build_insert(
                        agency_code,
                        or.clone(),
                        table_name.clone(),
                        columns.clone(),
                        overwrite.clone(),
                        source.clone(),
                        partitioned.clone(),
                        after_columns.clone(),
                        table.clone(),
                        on.clone(),
                    ) {
                        Some(sql) => {
                            return sql;
                        }
                        None => println!(),
                    }
                }
            }
            _ => {}
        }
    }
    up_sql
}
//构建租户化insert语句
fn build_insert(
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
        value: config.tenant.column.clone(),
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
//构建租户化select
fn build_select(select: Select, agency_code: String) -> String {
    let config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    //克隆一份做备份
    let mut select1 = select.clone();
    //处理租户字段
    select1.selection = match select.selection.clone() {
        //如果原SQL已经有查询条件 则直接加上 租户化条件
        Some(selection) => Option::from(Expr::BinaryOp {
            left: Box::new(selection),
            op: BinaryOperator::And,
            right: Box::new(Expr::BinaryOp {
                left: Box::new(Expr::Identifier(Ident {
                    value: config.tenant.column.clone(),
                    quote_style: None,
                })),
                op: BinaryOperator::Eq,
                right: Box::new(Expr::Value(Text::SingleQuotedString(agency_code))),
            }),
        }),
        //如果原始sql里没有查询条件 则直接加上租户化条件
        None => Some(Expr::BinaryOp {
            left: Box::new(Expr::Identifier(Ident {
                value: config.tenant.column.clone(),
                quote_style: None,
            })),
            op: BinaryOperator::Eq, //这里是 =
            right: Box::new(Expr::Value(Text::SingleQuotedString(agency_code))),
        }),
    };
    return select1.to_string();
}

//递归查找 租户列是否存在 存在则返回true
fn deep_find(selection: &Expr) -> bool {
    let config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    let column = config.tenant.column.clone();
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
