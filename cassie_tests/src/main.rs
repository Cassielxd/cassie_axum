use sqlparser::{
    ast::{Ident, Query, SetExpr, Value, Values},
    dialect::GenericDialect,
    parser::Parser,
};

fn main() {
    let sql = "insert into sys_dict_type (dict_type,dict_name,remark,sort,creator,create_date,updater,update_date) values (?,?,?,?,?,?,?,?)";
    let dialect = GenericDialect {}; // or AnsiDialect, or your own dialect ...
                                     //解析sql
    let atc = Parser::parse_sql(&dialect, &sql).unwrap();

    if let Some(data) = atc.get(0) {
        match data {
            sqlparser::ast::Statement::Insert {
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
                let mut c = columns.clone();
                let agency_code = Ident {
                    value: "agency_code".to_string(),
                    quote_style: None,
                };
                if !columns.contains(&agency_code) {
                    c.push(agency_code);
                    match source.body.clone() {
                        sqlparser::ast::SetExpr::Values(v) => {
                            let mut a = v.0.clone();
                            let mut a1 = v.0.get(0).unwrap().clone();
                            a1.push(sqlparser::ast::Expr::Value(Value::SingleQuotedString(
                                "?".to_string(),
                            )));
                            let query = Query {
                                body: SetExpr::Values(Values(vec![a1])),
                                ..*source.clone()
                            };
                            let sql1 = sqlparser::ast::Statement::Insert {
                                or: or.clone(),
                                table_name: table_name.clone(),
                                columns: c,
                                overwrite: overwrite.clone(),
                                source: Box::new(query),
                                partitioned: partitioned.clone(),
                                after_columns: after_columns.clone(),
                                table: table.clone(),
                                on: on.clone(),
                            };
                            println!("{}", sql1.to_string());
                        }
                        _ => todo!(),
                    }
                }
            }
            _ => {}
        }
    }
}
