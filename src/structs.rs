use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Response<T> {
    pub data: T,
    pub retcode: i32,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub post_type: String,
    pub message_type: String,
    pub time: i32,
    pub self_id: i64,
    pub sub_type: String,
    pub raw_message: String,
    pub sender: Sender,
    pub message: String,
    pub message_seq: i64,
    pub user_id: i64,
    pub message_id: i32,
    pub anonymous: Option<Anonymous>,
    pub font: i32,
    pub group_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sender {
    pub age: i32,
    pub area: String,
    pub card: String,
    pub level: String,
    pub nickname: String,
    pub role: String,
    pub sex: String,
    pub title: String,
    pub user_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Anonymous {
    pub id: i64,
    pub name: String,
    pub flag: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginInfo {
    pub nickname: String,
    pub user_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GroupMsgHistory {
    pub messages: Vec<Message>,
}
