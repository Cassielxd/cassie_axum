use rbatis::crud::CRUD;
use rbatis::rbatis::Rbatis;
#[macro_use]
extern crate rbatis;

#[crud_table(table_name:information_schema.tables)]
#[derive(Clone, Debug)]
pub struct Tables {
    pub table_comment: Option<String>,
    pub table_name: Option<String>,
}

#[crud_table(table_name:information_schema.COLUMNS)]
#[derive(Clone, Debug)]
pub struct TableColums {
    pub column_name: Option<String>,
    pub column_comment: Option<String>,
    pub data_type: Option<String>,
}

#[tokio::main]
async fn main() {
    let rbatis = Rbatis::new();
    rbatis
        .link("mysql://root:123456@localhost:3306/rivet_admin")
        .await
        .expect("rbatis link database fail!");
    let result = rbatis.fetch_list::<Tables>().await.unwrap();

    println!("{:#?}", result);
}

async fn text() {
    let url = format!("https://api.weixin.qq.com/sns/jscode2session?appid={appid}&secret={secret}&js_code={code}&grant_type=authorization_code",
    appid="",
    code="",
    secret=""
);
    let client = reqwest::Client::new();
    let result = client.get(url).send().await.unwrap().text().await;
    println!("{:#?}", result);
}
