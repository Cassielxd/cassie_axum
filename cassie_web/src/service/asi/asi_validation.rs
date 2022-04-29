use std::collections::HashMap;

use cassie_common::error::Error;
use cassie_common::error::Result;
use cassie_domain::dto::asi_dto::AsiGroupColumnDTO;

pub fn validate_value(columns: &Vec<AsiGroupColumnDTO>, values: &HashMap<String, String>) -> Result<()> {
  for cloumn in columns {
    if cloumn.is_required().clone().unwrap() == "Y".to_string() && !values.contains_key(&cloumn.column_code().clone().unwrap()) {
      let msg = format!("列{}不能为空!", cloumn.column_code().clone().unwrap());
      return Err(Error::E(msg));
    }
  }
  Ok(())
}

pub fn validate_values(columns: &Vec<AsiGroupColumnDTO>, values: &Vec<HashMap<String, String>>) -> Result<()> {
  for va in values {
    if let Err(e) = validate_value(columns, va) {
      return Err(e);
    }
  }
  Ok(())
}
