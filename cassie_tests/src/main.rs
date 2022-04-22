use rbatis::rbatis::Rbatis;

#[tokio::main]
async fn main() {
    let rbatis = Rbatis::new();

    rbatis
        .link("mysql://root:123456@localhost:3306/rivet_admin")
        .await
        .expect("rbatis link database fail!");
      let result =   rbatis.exec("SHOW TABLES", Vec::new()).await.unwrap();

      println!("{:#?}", result);
}

async fn text(){
    let url = format!("https://api.weixin.qq.com/sns/jscode2session?appid={appid}&secret={secret}&js_code={code}&grant_type=authorization_code",
    appid="",
    code="",
    secret=""
);
let client = reqwest::Client::new();
let result= client.get(url).send().await.unwrap()
.text()
.await;
println!("{:#?}",result);
}