use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ErrorMessage {
    pub r#type: String,
    pub message: String,
    pub reason: String,
}

#[derive(Debug, Deserialize)]
pub struct SubscriptionMessage {
    pub r#type:String,
    pub channels:Vec<SubscriptionMessageChannels>,
}

#[derive(Debug, Deserialize)]
pub struct SubscriptionMessageChannels {
    pub name: String,
    pub product_ids: Vec<String>,
    pub account_id: Option<String>,
}
