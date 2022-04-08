use crate::APPLICATION_CONTEXT;
use cached::proc_macro::cached;
use cassie_config::config::ApplicationConfig;
use cassie_domain::request::RequestModel;
use rbatis::plugin::intercept::SqlIntercept;
use rbatis::rbatis::Rbatis;
use rbatis::Error;
use rbson::Bson;
use sqlparser::ast::Statement::Insert;
use sqlparser::ast::Statement::Query as Iquwey;
use sqlparser::ast::{
    BinaryOperator, Expr, Ident, ObjectName, Query, SetExpr, TableWithJoins, Value as Text, Values,
};
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
//租户化拦截器
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
            let request_model = APPLICATION_CONTEXT.get_local::<RequestModel>();
            //修改租户化方式直接拼接 不可更该顺序
            *sql = build(sql.clone(), request_model.agency_code.clone());
            //添加租户化条件
        }
        return Ok(());
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn intercept_query(from: Vec<TableWithJoins>) -> bool {
    let config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    for elem in from.iter() {
        let relation = &elem.relation;
        match relation {
            sqlparser::ast::TableFactor::Table {
                name,
                alias,
                args,
                with_hints,
            } => {
                let ObjectName(table_info) = name.clone();
                let tableinfo = table_info.get(0).unwrap();
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
#[cached(time = 60, size = 100)]
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
                    let select1 = select.clone().from;
                    if intercept_query(select1) {
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
                                    right: Box::new(Expr::Value(Text::SingleQuotedString(
                                        agency_code,
                                    ))),
                                }),
                            }),
                            //如果原始sql里没有查询条件 则直接加上租户化条件
                            None => Some(Expr::BinaryOp {
                                left: Box::new(Expr::Identifier(Ident {
                                    value: config.tenant.column.clone(),
                                    quote_style: None,
                                })),
                                op: BinaryOperator::Eq,
                                right: Box::new(Expr::Value(Text::SingleQuotedString(agency_code))),
                            }),
                        };
                        return select1.to_string();
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
                if intercept_insert(table_name.clone()) {
                    let mut c = columns.clone();
                    let agency_column = Ident {
                        value: config.tenant.column.clone(),
                        quote_style: None,
                    };
                    if !columns.contains(&agency_column) {
                        c.push(agency_column);
                        match &source.body {
                            sqlparser::ast::SetExpr::Values(value) => {
                                let mut a1 = value.0.get(0).unwrap().clone();
                                a1.push(sqlparser::ast::Expr::Value(Text::SingleQuotedString(
                                    agency_code,
                                )));
                                let insert = Insert {
                                    or: or.clone(),
                                    table_name: table_name.clone(),
                                    columns: c,
                                    overwrite: overwrite.clone(),
                                    source: Box::new(Query {
                                        body: SetExpr::Values(Values(vec![a1])),
                                        ..*source.clone()
                                    }),
                                    partitioned: partitioned.clone(),
                                    after_columns: after_columns.clone(),
                                    table: table.clone(),
                                    on: on.clone(),
                                };
                                return insert.to_string();
                            }
                            _ => println!(),
                        }
                    }
                }
            }
            _ => {}
        }
    }
    up_sql
}
