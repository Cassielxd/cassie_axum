use std::collections::HashMap;

fn get_cloumn_map() -> HashMap<String, String> {
    //mysql column type to rust field type
    let mut map: HashMap<String, String> = HashMap::new();
    map.insert("int".to_string(), "i32".to_string());
    map.insert("tinyint".to_string(), "i32".to_string());
    map.insert("smallint".to_string(), "i32".to_string());
    map.insert("mediumint".to_string(), "i32".to_string());
    map.insert("bigint".to_string(), "i64".to_string());
    map.insert("float".to_string(), "f32".to_string());
    map.insert("double".to_string(), "f64".to_string());
    map.insert("decimal".to_string(), "f64".to_string());
    map.insert("bit".to_string(), "Vec<u8>".to_string());
    map.insert("char".to_string(), "String".to_string());
    map.insert("varchar".to_string(), "String".to_string());
    map.insert("tinytext".to_string(), "String".to_string());
    map.insert("text".to_string(), "String".to_string());
    map.insert("mediumtext".to_string(), "String".to_string());
    map.insert("longtext".to_string(), "String".to_string());

    map.insert("date".to_string(), "String".to_string());
    map.insert("datetime".to_string(), "String".to_string());
    map.insert("timestamp".to_string(), "String".to_string());
    map.insert("time".to_string(), "String".to_string());
    map.insert("year".to_string(), "i32".to_string());

    map.insert("enum".to_string(), "String".to_string());
    map.insert("set".to_string(), "String".to_string());
    map.insert("binary".to_string(), "String".to_string());
    map.insert("varbinary".to_string(), "String".to_string());

    map.insert("tinyblob".to_string(), "String".to_string());
    map.insert("blob".to_string(), "String".to_string());
    map.insert("mediumblob".to_string(), "String".to_string());
    map.insert("longblob".to_string(), "String".to_string());
    map
}
