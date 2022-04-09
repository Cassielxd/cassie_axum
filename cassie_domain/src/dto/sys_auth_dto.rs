/// 授权
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct SysAuthDTO {
    access_token: String,
    path: String,
}
