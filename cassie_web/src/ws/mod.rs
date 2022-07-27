pub mod msg;
pub mod ws_handle;
pub mod ws_server;

use cassie_domain::vo::jwt::JWTToken;
use futures_channel::mpsc::{unbounded, UnboundedSender};
use std::{
    collections::HashMap,
    io::Error as IoError,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio_tungstenite::tungstenite::protocol::Message;

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;
type UidMap = Arc<Mutex<HashMap<String, SocketAddr>>>;
type AddrMap = Arc<Mutex<HashMap<SocketAddr, String>>>;
type UserMap = Arc<Mutex<HashMap<SocketAddr, JWTToken>>>;

lazy_static! {
    pub static ref PEER_MAP: Arc<Mutex<HashMap<SocketAddr, Tx>>> = PeerMap::new(Mutex::new(HashMap::new()));
    pub static ref UID_MAP: Arc<Mutex<HashMap<String, SocketAddr>>> = UidMap::new(Mutex::new(HashMap::new()));
    pub static ref ADDR_MAP: Arc<Mutex<HashMap<SocketAddr, String>>> = AddrMap::new(Mutex::new(HashMap::new()));
    pub static ref USER_MAP: Arc<Mutex<HashMap<SocketAddr, JWTToken>>> = UserMap::new(Mutex::new(HashMap::new()));
}
