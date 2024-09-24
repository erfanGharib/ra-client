mod websocket;
mod utils;

use tokio;
use websocket::index::init_websocket;

#[tokio::main]
async fn main() {
    init_websocket();
}
