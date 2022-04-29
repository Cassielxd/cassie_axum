pub mod mapping;
pub use mapping::*;
#[crud_table(table_name:information_schema.tables)]
#[derive(Clone, Debug)]
pub struct Tables {
    pub table_comment: Option<String>,
    pub table_name: Option<String>,
}

#[crud_table(table_name:information_schema.COLUMNS)]
#[derive(Clone, Debug)]
pub struct TableColumns {
    pub column_name: Option<String>,
    pub column_comment: Option<String>,
    pub data_type: Option<String>,
}

#[derive(serde::Serialize, Debug)]
pub struct Field {
    pub field_name: String,
    pub field_type: String,
    pub comment: String,
}

impl From<TableColumns> for Field {
    fn from(arg: TableColumns) -> Self {
        let cloumn_map = get_cloumn_map();
        Self {
            field_name: arg.column_name.unwrap(),
            field_type: cloumn_map.get(&arg.data_type.unwrap()).unwrap().to_string(),
            comment: arg.column_comment.unwrap(),
        }
    }
}
