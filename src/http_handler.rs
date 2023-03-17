#![allow(dead_code)]
#![allow(unused_variables)]

use crate::api_struct;
use reqwest::header::CONTENT_TYPE;

#[derive(Debug)]
pub struct HttpHandler {
    host: String,
    port: u16,
    token: String,
}

impl HttpHandler {
    /// 获取登录QQ信息
    pub fn get_bot_qq(&self) -> u64 {
        let res = self.http_get("get_login_info");
        let login_info: api_struct::LoginInfo = serde_json::from_str(&res).unwrap();
        login_info.data.user_id
    }

    /// 获取指定群历史消息
    pub fn get_latest_group_msgs(&self, group_id: u64) {
        let group_info = api_struct::GroupInfo { group_id };
        let request_text = serde_json::to_string(&group_info).unwrap();
        let res = self.http_post("get_group_msg_history", request_text);
        // TODO: 进一步处理返回
    }

    /// 发送Get请求返回内容字符串
    fn http_get(&self, action: &str) -> String {
        let url = format!(
            "http://{}:{}/{}?access_token={}",
            self.host, self.port, action, self.token
        );
        reqwest::blocking::get(url).unwrap().text().unwrap()
    }

    /// 发送Post请求返回内容字符串
    fn http_post(&self, action: &str, data: String) -> String {
        let url = format!(
            "http://{}:{}/{}?access_token={}",
            self.host, self.port, action, self.token
        );
        let client = reqwest::blocking::Client::new();
        let res = client
            .post(url)
            .header(CONTENT_TYPE, "application/json")
            .body(data)
            .send()
            .unwrap();
        res.text().unwrap()
    }

}

impl Default for HttpHandler {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 5700,
            token: "".to_string(),
        }
    }
}

impl HttpHandler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn host(&self) -> &String {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn token(&self) -> &String {
        &self.token
    }
}

#[test]
fn test_get_bot_qq() {
    let mut handler = HttpHandler::new();
    handler.token = "WERTYUIO".to_string();
    let qq = handler.get_bot_qq();
}

#[test]
fn test_get_latest_group_msgs() {
    let mut handler = HttpHandler::new();
    handler.token = "WERTYUIO".to_string();
    handler.get_latest_group_msgs(531241108);
}
