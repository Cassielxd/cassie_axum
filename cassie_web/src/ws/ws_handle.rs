use crate::ws::msg::{Msg, MsgBody, MsgType};
use crate::ws::{ADDR_MAP, PEER_MAP, UID_MAP, USER_MAP};
use cassie_domain::vo::jwt::JWTToken;
use std::net::SocketAddr;
use tokio_tungstenite::tungstenite::Message;

//消息处理
pub fn handle_msg(addr: SocketAddr, msg: String) {
    println!("{}", msg);
    let mut massage: Msg = serde_json::from_str(&*msg).unwrap();
    match massage.mt {
        MsgType::Ping => {
            //心跳检测
        }
        MsgType::Msg => {
            //消息发送 默认发给所有人
            {
                let usermap = USER_MAP.clone();
                let p = PEER_MAP.clone();
                let peers = p.lock().unwrap();
                let userinfomap = usermap.lock().unwrap();
                let indfo = userinfomap.get(&addr).unwrap();
                let mut body = massage.body.clone().unwrap();
                body.from = indfo.username().clone();
                massage.body = Some(body);
                // 把消息发送给 非自己的所有用户
                let broadcast_recipients = peers.iter().filter(|(peer_addr, _)| peer_addr != &&addr).map(|(_, ws_sink)| ws_sink);
                for recp in broadcast_recipients {
                    let m = serde_json::to_string(&massage).unwrap();
                    recp.unbounded_send(Message::from(m)).unwrap();
                }
            }
        }
        MsgType::UserList => {
            //获取在线用户信息
            {
                let usermap = USER_MAP.clone();
                let userinfomap = usermap.lock().unwrap();
                let list: Vec<JWTToken> = userinfomap.iter().map(|(_, v)| v.clone()).collect();
                let p = PEER_MAP.clone();
                let peers = p.lock().unwrap();
                let recp = peers.get(&addr).unwrap();
                let body = crate::ws::msg::MsgBody {
                    from: "".to_string(),
                    to: "".to_string(),
                    text: serde_json::to_string(&list).unwrap(),
                };
                massage.body = Some(body);
                let m = serde_json::to_string(&massage).unwrap();
                recp.unbounded_send(Message::from(m)).unwrap();
            }
        }
    }
}

//用户下线
pub fn off_line(addr: &SocketAddr) {
    let p = PEER_MAP.clone();
    let umap = UID_MAP.clone();
    let amap = ADDR_MAP.clone();
    let usermap = USER_MAP.clone();
    p.lock().unwrap().remove(addr);
    usermap.lock().unwrap().remove(addr);
    let uid = amap.lock().unwrap().remove(addr);
    match uid {
        None => {}
        Some(id) => {
            umap.lock().unwrap().remove(&id);
        }
    }
}

//向单个用户发送消息
fn send_msg(uid: String, msg: String) {
    let peer_map = PEER_MAP.clone();
    let umap = UID_MAP.clone();
    let mut mp = umap.lock().unwrap();
    let addr = mp.get(&*uid);
    match addr {
        None => {
            //用户不在线
        }
        Some(address) => {
            //如果用户在线 则发送消息
            if let Some(recp) = peer_map.lock().unwrap().get(address) {
                recp.unbounded_send(Message::from(msg.clone())).unwrap();
            }
        }
    }
}
