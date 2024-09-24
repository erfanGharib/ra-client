use tokio::net::TcpStream;
use futures_util::StreamExt;
use tokio;
use futures::stream::{SplitSink, SplitStream};
use tokio_tungstenite::{
    tungstenite::protocol::Message, 
    WebSocketStream,
    MaybeTlsStream,
    connect_async
};

use crate::utils;
use crate::websocket::events::handle_events::handle_events;


pub type TWsSender = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;
pub type TWsReceiver = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

// TODO: read configs from config file
#[tokio::main]
pub async fn init_websocket() {
    let delay_time: u64 = 10; // 10 seconds
    let url = "ws://localhost:5000";
    let (ws_stream, _) = 
        connect_async(url)
        .await
        .expect("Failed to connect");
    let (ws_sender, ws_receiver) = 
        ws_stream
        .split();

    println!("Connected to the WebSocket server!");

    handle_events(&ws_sender, &ws_receiver).await;

    // utils::run_with_delay(
    //     // () -> handle_events(&ws_sender, &ws_receiver), 
    //     async || -> () { handle_events(&ws_sender, &ws_receiver).await }, 
    //     delay_time
    // );
}
