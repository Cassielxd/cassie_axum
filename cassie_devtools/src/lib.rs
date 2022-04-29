#[macro_use]
extern crate rbatis;
pub mod core;
pub use crate::core::*;
use handlebars::Handlebars;
use rbatis::{crud::CRUD, rbatis::Rbatis};

use handlebars::to_json;
use serde_json::value::{Map, Value as Json};
use std::fs::File;

const TEMP: [(&str, &str, &str); 4] = [
  ("service_template", "service.rs", "./cassie_devtools/src/template/service_template.hbs"),
  ("resource_template", "resource.rs", "./cassie_devtools/src/template/resources_template.hbs"),
  ("dto_template", "dto.rs", "./cassie_devtools/src/template/dto_template.hbs"),
  ("entity_template", "entity.rs", "./cassie_devtools/src/template/entity_template.hbs"),
];

pub async fn init_rbatis(database_url: &str) -> Rbatis {
  let rbatis = Rbatis::new();
  rbatis.link(database_url).await.expect("rbatis link database fail!");
  return rbatis;
}

pub async fn get_columns(table_name: &str) -> Vec<Field> {
  let rb = init_rbatis("mysql://root:root@localhost:3306/rivet_admin").await;
  let w = rb.new_wrapper().eq("table_name", table_name).eq("table_schema", "rivet_admin");
  let columns = rb.fetch_list_by_wrapper::<TableColumns>(w).await.unwrap();
  columns.into_iter().map(|c| c.into()).collect()
}

pub fn render_template(table_name: &str, data: &Map<String, Json>) {
  let mut handlebars = Handlebars::new();
  for (temp_name, file_path, temple_path) in TEMP {
    handlebars.register_template_file(temp_name, temple_path).unwrap();
    let mut output__dtofile = File::create(format!("target/{}_{}", table_name, file_path.to_string())).unwrap();
    handlebars.render_to_write(temp_name, &data, &mut output__dtofile).unwrap();
  }
}
