use std::{error::Error, str::FromStr};
use tokio_tungstenite::tungstenite;


pub mod orderbook_update;
pub mod public_trade;
pub mod response;

#[derive(Debug)]
pub enum KalshiSocketMessage {
    // Textual messages
    SubscribedResponse(response::SubscribedResponse), // response to a sent message indicating success
    ErrorResponse(response::ErrorResponse), // response to a sent message indicating failure
    OrderbookSnapshot(orderbook_update::OrderbookSnapshot), // snapshot of orderbook, first message from a orderbook_delta subscription
    OrderbookDelta(orderbook_update::OrderbookDelta),       // orderbook change
    TradeUpdate(public_trade::TradeUpdate),                 // trade executed between two parties
    // Heartbeat / Close
    Ping,
    Pong,
    // Unexpected Types From Kalshi API
    Binary(tungstenite::Bytes),
    Frame(tungstenite::protocol::frame::Frame),
    Close(Option<tungstenite::protocol::frame::CloseFrame>),
}

impl KalshiSocketMessage {
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

    fn from_textual_message(s: String) -> Result<KalshiSocketMessage, Box<dyn Error>> {
        let msg_type = determine_type(&s.clone()).ok_or("could not determine message type")?;
        let socket_message = match msg_type.as_str() {
            "subscribed" => {
                let inner: response::SubscribedResponse = serde_json::from_str(&s)?;
                KalshiSocketMessage::SubscribedResponse(inner)
            }
            "orderbook_snapshot" => {
                let inner: orderbook_update::OrderbookSnapshot = serde_json::from_str(&s)?;
                KalshiSocketMessage::OrderbookSnapshot(inner)
            }
            "orderbook_delta" => {
                let inner: orderbook_update::OrderbookDelta = serde_json::from_str(&s)?;
                KalshiSocketMessage::OrderbookDelta(inner)
            }
            "trade" => {
                let inner: public_trade::TradeUpdate = serde_json::from_str(&s)?;
                KalshiSocketMessage::TradeUpdate(inner)
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
