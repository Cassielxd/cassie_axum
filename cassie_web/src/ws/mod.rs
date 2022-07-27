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

///-----------------------需要优化-----------------------
///目前阶段 把所有在线用户当成一个 room
/// 如果想要扩展 在参数传递上可以 绑定room_id
/// 然后根据
/// 例子： pub static ref ROM_MAP: Arc<Mutex<HashMap<String, Vec<SocketAddr>>>> = RoomMap::new(Mutex::new(HashMap::new()));
///  key 是rooid  value 是一个集合 包含的是这个房间的所有人
/// 发送的时候根据 是rooid找到集合 然后根据SocketAddr找到对应的 Tx依次发送
///----------------------------------------------
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
