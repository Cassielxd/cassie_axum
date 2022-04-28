use cassie_devtools::{get_columns, render_template};
use handlebars::to_json;
use serde_json::Map;

#[tokio::main]
async fn main() {
  let table_name = "sys_group_data";
  let model_name = "SysGroupData";
  let model = "sys_group_data";
  let columns = get_columns(table_name).await;
  let mut data = Map::new();
  data.insert("model".to_string(), to_json(model));
  data.insert("model_name".to_string(), to_json(model_name));
  data.insert("table_name".to_string(), to_json(table_name));
  data.insert("columns".to_string(), to_json(&columns));
  render_template(table_name, &data);
}
