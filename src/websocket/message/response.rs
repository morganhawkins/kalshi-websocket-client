use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SubscribedResponse {
    pub r#type: String,
    pub id: u64,
    pub msg: SubscribedResponseMessage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubscribedResponseMessage {
    pub channel: String,
    pub sid: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub id: u64,
    pub code: u64,
    pub msg: ErrorResponseMessage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponseMessage {
    pub code: u64,
    pub msg: String,
}
