#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Debug)]
pub struct WsHandler {
    host: String,
    port: i32,
    token: String,
}

impl Default for WsHandler {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            token: "".to_string(),
        }
    }
}

impl WsHandler {
    /// 默认生成
    pub fn new() -> Self {
        Self::default()
    }

    pub fn connect(&self) {
        let addr = format!(
            "ws://{}:{}/?access_token={}",
            self.host, self.port, self.token
        );
    }

    pub fn connect_api(&self) {
        let addr = format!(
            "ws://{}:{}/api?access_token={}",
            self.host, self.port, self.token
        );
    }
}

#[test]
fn test_default() {
    let mut default_handler = WsHandler::default();
    default_handler.port = 8080;
    println!("{:?}", default_handler);
    assert_eq!(8080, default_handler.port);
}
