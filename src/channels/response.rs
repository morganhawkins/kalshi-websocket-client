use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SubscribedResponse {
    pub r#type: String,
    pub id: u64,
    pub msg: SubscribedResponseMessage
}

#[derive(Deserialize, Debug)]
struct SubscribedResponseMessage {
    pub channel: String,
    pub sid: u64,
}

#[derive(Deserialize, Debug)]
pub struct ErrorResponse {
    pub id: u64,
    pub code: u64,
    pub msg: ErrorResponseMessage,
}

#[derive(Deserialize, Debug)]
struct ErrorResponseMessage {
    pub code: u64,
    pub msg: String,
}