#![allow(dead_code)]
#![allow(unused_variables)]

use tokio_tungstenite::connect_async;

#[tokio::test]
async fn main() {
    let connect_addr = "ws://127.0.0.1:8080/?access_token=WERTYUIO".to_string();
    let (ws_stream, res) = connect_async(connect_addr).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");
    println!("{:?}", res);
    assert_eq!(101, res.status());
}