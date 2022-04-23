
use std::fs::File;
use serde_json::value::{Map, Value as Json};
use handlebars::{Handlebars, to_json};
use cassie_devtools::core::Field;

fn main() {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_file("entity_template", "./cassie_devtools/src/template/entity_template.hbs")
        .unwrap();
    let mut data =Map::new();
    data.insert("model".to_string(),to_json("user"));
    data.insert("model_name".to_string(),to_json("User"));
    data.insert("table_name".to_string(),to_json("user"));
    let columns = vec![Field{ field_name: "id".to_string(), field_type: "String".to_string() }];
    data.insert("columns".to_string(),to_json(&columns));
    let mut output_file = File::create("target/entity_resource.rs").unwrap();
    handlebars.render_to_write("entity_template", &data, &mut output_file).unwrap();
}
/*
//resource 生成
handlebars
        .register_template_file("resource_template", "./cassie_devtools/src/template/resources_template.hbs")
        .unwrap();
   let mut data =HashMap::new();
    data.insert("model","user");
    data.insert("model_name","User");
    data.insert("table_name","user");
    let mut output_file = File::create("target/resource.rs").unwrap();
    handlebars.render_to_write("resource_template", &data, &mut output_file).unwrap();


//entity 生成
  handlebars
        .register_template_file("entity_template", "./cassie_devtools/src/template/entity_template.hbs")
        .unwrap();
    let mut data =Map::new();
    data.insert("model".to_string(),to_json("user"));
    data.insert("model_name".to_string(),to_json("User"));
    data.insert("table_name".to_string(),to_json("user"));
    let columns = vec![Field{ field_name: "id".to_string(), field_type: "String".to_string() }];
    data.insert("columns".to_string(),to_json(&columns));
    let mut output_file = File::create("target/entity_resource.rs").unwrap();
    handlebars.render_to_write("entity_template", &data, &mut output_file).unwrap();
*/
