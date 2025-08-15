use std::{error::Error, str::FromStr};
use tokio_tungstenite::tungstenite;

pub mod client;
pub mod ticker_updates;
pub mod response;

#[derive(Debug)]
pub enum CoinbaseSocketMessage {
    // Textual messages
    TickerUpdate(ticker_updates::TickerUpdate),
    ErrorMessage(response::ErrorMessage),
    SubscriptionMessage(response::SubscriptionMessage),
    // Heartbeat / Close
    Ping,
    Pong,
    // Unexpected Types From Kalshi API
    Binary(tungstenite::Bytes),
    Frame(tungstenite::protocol::frame::Frame),
    Close(Option<tungstenite::protocol::frame::CloseFrame>),
}

impl CoinbaseSocketMessage {
    pub fn from_message(s: tungstenite::Message) -> Result<Self, Box<dyn Error>> {
        match s {
            tungstenite::Message::Text(text) => Self::from_textual_message(text.to_string()),
            tungstenite::Message::Ping(_) => Ok(Self::Ping),
            tungstenite::Message::Pong(_) => Ok(Self::Pong),
            tungstenite::Message::Binary(b) => Ok(Self::Binary(b)),
            tungstenite::Message::Close(c) => Ok(Self::Close(c)),
            tungstenite::Message::Frame(f) => Ok(Self::Frame(f)),
        }
    }

    fn from_textual_message(s: String) -> Result<CoinbaseSocketMessage, Box<dyn Error>> {
        let msg_type = determine_type(&s.clone()).ok_or("could not determine message type")?;
        let socket_message = match msg_type.as_str() {
            "subscriptions" => {
                let inner: response::SubscriptionMessage = serde_json::from_str(&s)?;
                CoinbaseSocketMessage::SubscriptionMessage(inner)
            }
            "error" => {
                let inner: response::ErrorMessage = serde_json::from_str(&s)?;
                CoinbaseSocketMessage::ErrorMessage(inner)
            }
            "ticker" => {
                let inner: ticker_updates::TickerUpdate = serde_json::from_str(&s)?;
                CoinbaseSocketMessage::TickerUpdate(inner)
            }
            "heartbeat" => {
                CoinbaseSocketMessage::Ping
            }
            _ => return Err(format!("unrecognized textual message type {s}").into()),
        };
        Ok(socket_message)
    }
}

// TODO: make this faster with regex, do not deserialize the whole string, don't clone so much
fn determine_type(msg: &str) -> Option<String> {
    let msg_object = serde_json::Value::from_str(&msg).ok()?;

    let msg_type_value = match msg_object {
        serde_json::Value::Object(obj) => {
            let val = obj.get("type")?;
            val.clone()
        }
        _ => return None,
    };

    match msg_type_value {
        serde_json::Value::String(s) => return Some(s),
        _ => return None,
    }
}
