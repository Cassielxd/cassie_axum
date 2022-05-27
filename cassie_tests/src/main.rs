use cassie_event::api_operation;

#[api_operation]
pub fn test(name:impl Into<String>) -> String {
    println!("Hello, {}!", name.into());
    "hello".to_string()
}

fn main() {
    let t =test("lixingdong"); 
}
