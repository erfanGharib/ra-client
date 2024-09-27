use crate::utils;
extern crate ws;
use ws::{connect, CloseCode, Handler, Handshake, Message, Result, Sender};
// use super::events::handle_events::handle_events;

struct Client {
    out: Sender,
}

impl Handler for Client {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        self.out.send("Hello, WebSocket Server!")
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("Client got message: '{}'.", msg);

        self.out.close(CloseCode::Normal)
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("WebSocket closed with code {:?} and reason: {}", code, reason);
    }
}

pub fn init_websocket() {
    // TODO: put these vars in separate config file
    let ws_url = "ws://localhost:5000";
    let delay_time = 100;

    utils::run_with_delay(
        || connect(ws_url, |out| Client { out }).unwrap(), 
        delay_time
    );
}