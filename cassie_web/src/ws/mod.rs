pub mod msg;
pub mod ws_handle;
pub mod ws_server;

use cassie_domain::vo::jwt::JWTToken;
use futures_channel::mpsc::UnboundedSender;
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio_tungstenite::tungstenite::protocol::Message;

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;
type UidMap = Arc<Mutex<HashMap<String, SocketAddr>>>;
type AddrMap = Arc<Mutex<HashMap<SocketAddr, String>>>;
type UserMap = Arc<Mutex<HashMap<SocketAddr, JWTToken>>>;

//需要优化
lazy_static! {
    //Map<"地址ip"，“发送通道”>
    pub static ref PEER_MAP: Arc<Mutex<HashMap<SocketAddr, Tx>>> = PeerMap::new(Mutex::new(HashMap::new()));
    //Map<"用户id"，“地址ip”>
    pub static ref UID_MAP: Arc<Mutex<HashMap<String, SocketAddr>>> = UidMap::new(Mutex::new(HashMap::new()));
    //Map<“地址ip"，"用户id”>
    pub static ref ADDR_MAP: Arc<Mutex<HashMap<SocketAddr, String>>> = AddrMap::new(Mutex::new(HashMap::new()));
    //Map<“地址ip"，"用户信息”>
    pub static ref USER_MAP: Arc<Mutex<HashMap<SocketAddr, JWTToken>>> = UserMap::new(Mutex::new(HashMap::new()));
}
