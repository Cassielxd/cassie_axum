use std::collections::HashMap;

use crate::dto::asi_dto::{AsiGroupValuesDTO, AsiGroupColumnDTO};
use cassie_common::error::Error;
use cassie_common::error::Result;

pub fn validate_values(columns: &Vec<AsiGroupColumnDTO>,values: &Vec<AsiGroupValuesDTO>)->Result<()>{
      let mut v_map = HashMap::<String,AsiGroupValuesDTO>::new();
    for value  in  values{
        v_map.insert(value.column_code.clone().unwrap(), value.clone());
    }
    for cloumn in columns  {
        if cloumn.is_required.clone().unwrap()=="Y".to_string() && !v_map.contains_key(&cloumn.column_code.clone().unwrap()){
            let msg = format!("{}列定义不能为空!",cloumn.column_code.clone().unwrap());
            return Err(Error::E(msg));
        }
    }
    Ok(())
}