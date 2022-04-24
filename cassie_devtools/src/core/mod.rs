pub mod mapping;
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

#[derive(serde::Serialize)]
pub struct Field {
    pub field_name: String,
    pub field_type: String,
}
