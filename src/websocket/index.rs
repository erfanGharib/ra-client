use crate::utils;
extern crate ws;
use ws::{connect, CloseCode, Handler, Handshake, Message, Result, Sender, Error};

// TODO: put these vars in separate config file
static ws_url: &str = "ws://localhost:5000";
static mut delay_time: u64 = 10;
static max_delay_time: u64 = 300; // 5 minutes

struct Client {
    out: Sender,
}

impl Handler for Client {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        unsafe {
            delay_time = 10;
        }

        self.out.send("Hello, WebSocket Server!")
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("Client got message: '{}'.", msg);

        Ok(())
    }

    fn on_error(&mut self, err: Error) {
        retry_for_connection();
    }

    fn on_close(&mut self, code: CloseCode, _reason: &str) {
        retry_for_connection();        
    }
}

fn retry_for_connection() {
    unsafe {
        if delay_time < max_delay_time {
            delay_time = utils::convert_f32_to_u64(delay_time as f32 * 1.5);
        }

        println!("WebSocket connection closed.");
        println!("Retrying in {} seconds...\n", delay_time);
        
        utils::run_with_delay(
            || connect(ws_url, |out| Client { out }).unwrap(), 
            delay_time
        );        
    }
}

pub fn init_websocket() {
    println!("Attempting to connect to {}\n", ws_url);
    connect(ws_url, |out| Client { out });
}