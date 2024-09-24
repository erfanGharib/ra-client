use tokio_tungstenite::tungstenite::protocol::Message;
use crate::websocket::index::{init_websocket, TWsReceiver, TWsSender};

use super::on_command_received::on_command_received;

pub async fn handle_events(
    ws_sender: &TWsSender, 
    ws_receiver: &mut TWsReceiver
) {
    while let Some(message) = ws_receiver.next().await {
        match message {
            Ok(Message::Text(raw_command)) => {
                on_command_received(&ws_sender, &raw_command);
            }
            Ok(Message::Binary(_)) => {
                println!("Received binary message");
            }
            Ok(Message::Close(_)) => {
                println!("Connection closed");
                init_websocket();
                break; // prevent from duplicate connection
            }
            Ok(Message::Ping(ping)) => {
                println!("Received Ping: {:?}", ping);
            }
            Ok(Message::Pong(pong)) => {
                println!("Received Pong: {:?}", pong);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                init_websocket();
                break; // prevent from duplicate connection
            }
            _ => {}
        }
    }
}