#![allow(dead_code)]
#![allow(unused_variables)]

use std::error::Error;

use reqwest::header::CONTENT_TYPE;
use reqwest::StatusCode;
use serde_json::json;

use crate::structs::*;

#[derive(Debug)]
pub struct HttpHandler {
    host: String,
    port: u16,
    token: String,
}

impl HttpHandler {
    /// 获取登录QQ信息
    pub fn get_login_info(&self) -> Result<LoginInfo, Box<dyn Error>> {
        let text = self.http_get("get_login_info")?;
        let res: Response<LoginInfo> = serde_json::from_str(&text)?;
        Ok(res.data)
    }

    /// 获取指定群历史消息
    pub fn get_group_msg_history(&self, group_id: u64) -> Result<GroupMsgHistory, Box<dyn Error>> {
        let data = json!({ "group_id": group_id }).to_string();
        let text = self.http_post("get_group_msg_history", data)?;
        let res: Response<GroupMsgHistory> = serde_json::from_str(&text)?;
        Ok(res.data)
    }

    /// 发送消息到指定群聊
    pub fn send_group_msg(&self, group_id: u64, message: &str) -> Result<String, Box<dyn Error>> {
        let data = json!({ "group_id": group_id, "message": message }).to_string();
        let text = self.http_post("send_group_msg", data)?;
        Ok(text)
    }

    /// 发送Get请求返回内容字符串
    pub fn http_get(&self, action: &str) -> Result<String, String> {
        let url = format!(
            "http://{}:{}/{}?access_token={}",
            self.host, self.port, action, self.token
        );
        let res = reqwest::blocking::get(url);
        if let Err(why) = res {
            return Err(format!("Get请求失败: {}, 终结点: {}", why, action));
        }
        let res = res.unwrap();
        match res.status() {
            StatusCode::OK => Ok(res.text().unwrap()),
            other => Err(format!("异常返回码: {}, 终结点: {}", other, action)),
        }
    }

    /// 发送Post请求返回内容字符串
    pub fn http_post(&self, action: &str, data: String) -> Result<String, String> {
        let url = format!(
            "http://{}:{}/{}?access_token={}",
            self.host, self.port, action, self.token
        );
        let client = reqwest::blocking::Client::new();
        let res = client
            .post(url)
            .header(CONTENT_TYPE, "application/json")
            .body(data)
            .send();
        if let Err(why) = res {
            return Err(format!("Post请求失败: {}, 终结点: {}", why, action));
        }
        let res = res.unwrap();
        match res.status() {
            StatusCode::OK => Ok(res.text().unwrap()),
            other => Err(format!("异常返回码: {}, 终结点: {}", other, action)),
        }
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

    pub fn host(&mut self, new_host: &str) -> &mut Self {
        self.host = new_host.to_string();
        self
    }

    pub fn port(&mut self, new_port: u16) -> &mut Self {
        self.port = new_port;
        self
    }

    pub fn token(&mut self, new_token: &str) -> &mut Self {
        self.token = new_token.to_string();
        self
    }
}
