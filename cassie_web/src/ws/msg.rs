use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MsgType {
    Ping,     //心跳
    UserList, //在线用户列表
    Msg,      //消息类型
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MsgBody {
    pub from: String, //来自谁
    pub to: String,   //发给谁
    pub text: String, //内容 字符串 有可能也是 json
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Msg {
    pub mt: MsgType, //消息类型
    pub body: Option<MsgBody>,
}
