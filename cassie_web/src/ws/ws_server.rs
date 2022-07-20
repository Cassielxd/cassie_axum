use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};
use log::info;
use std::{
    collections::HashMap,
    env,
    io::Error as IoError,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use crate::APPLICATION_CONTEXT;
use cassie_config::config::ApplicationConfig;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::tungstenite::{
    handshake::server::{ErrorResponse, Request, Response},
    http::StatusCode,
};

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

async fn handle_connection(peer_map: PeerMap, raw_stream: TcpStream, addr: SocketAddr) {
    let ws_stream = tokio_tungstenite::accept_hdr_async(raw_stream, |req: &Request, response: Response| Ok(response))
        .await
        .expect("websocket 握手发生错误");
    info!("WebSocket 连接成功: {}", addr);
    // 讲连接进来的客户端加入map方便全局使用
    let (tx, rx) = unbounded();
    peer_map.lock().unwrap().insert(addr, tx);
    let (outgoing, incoming) = ws_stream.split();

    let broadcast_incoming = incoming.try_for_each(|msg| {
        info!("接收到来自 {}: {}", addr, msg.to_text().unwrap());
        match msg.clone() {
            Message::Text(_ms) => {}
            Message::Binary(_) => {}
            Message::Ping(_) => {}
            Message::Pong(_) => {}
            Message::Close(_) => {}
        }
        let peers = peer_map.lock().unwrap();
        // 把消息发送给 非自己的所有用户
        let broadcast_recipients = peers.iter().filter(|(peer_addr, _)| peer_addr != &&addr).map(|(_, ws_sink)| ws_sink);
        for recp in broadcast_recipients {
            recp.unbounded_send(msg.clone()).unwrap();
        }
        future::ok(())
    });

    let receive_from_others = rx.map(Ok).forward(outgoing);
    pin_mut!(broadcast_incoming, receive_from_others);

    future::select(broadcast_incoming, receive_from_others).await;
    info!("{} 断开连接", &addr);
    peer_map.lock().unwrap().remove(&addr);
}

//实例化一个 ws server
pub async fn init_ws() -> Result<(), IoError> {
    let cassie_config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    //如果ws端口开启了 则启动 websocket
    if cassie_config.server().ws().is_some() {
        let addr = format!("{}:{}", cassie_config.server().host(), cassie_config.server().ws().clone().unwrap());
        let state = PeerMap::new(Mutex::new(HashMap::new()));
        let try_socket = TcpListener::bind(&addr).await;
        let listener = try_socket.expect("绑定失败");
        info!("Listening on: {}", addr);
        while let Ok((stream, addr)) = listener.accept().await {
            tokio::spawn(handle_connection(state.clone(), stream, addr));
        }
    }
    Ok(())
}
