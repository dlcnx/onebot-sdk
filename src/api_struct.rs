use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GroupInfo {
    pub group_id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct LoginInfoData {
    pub nickname: String,
    pub user_id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct LoginInfo {
    pub data: LoginInfoData,
    pub retcode: i32,
    pub status: String,
}