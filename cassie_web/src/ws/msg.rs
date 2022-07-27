use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MsgType {
    Ping,
    UserList,
    Msg,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MsgBody {
    pub from: String,
    pub to: String,
    pub text: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Msg {
    pub mt: MsgType,
    pub body: Option<MsgBody>,
}
