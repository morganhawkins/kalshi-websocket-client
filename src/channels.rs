use std::{error::Error, str::FromStr};
use serde::Deserialize;

pub mod client;
pub mod orderbook_updates;
pub mod public_trades;
pub mod response;

#[derive(Deserialize, Debug)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug)]
pub enum SocketMessage {
    SubscribedResponse(response::SubscribedResponse), // response to a sent message indicating success
    ErrorResponse(response::ErrorResponse), // response to a sent message indicating failure
    OrderbookSnapshot(orderbook_updates::OrderbookSnapshot), // snapshot of orderbook, first message from a orderbook_delta subscription
    OrderbookDelta(orderbook_updates::OrderbookDelta), // orderbook change 
    TradeUpdate(public_trades::TradeUpdate), // trade executed between two parties 
}

impl SocketMessage {
    pub fn from_str(s: String) -> Result<Self, Box<dyn Error>>{
        let msg_type = determine_type(&s.clone()).ok_or("could not determine message type")?;
        let socket_message = match msg_type.as_str() {
            "subscribed" => {
                let inner: response::SubscribedResponse = serde_json::from_str(&s)?; 
                SocketMessage::SubscribedResponse(inner)
            },
            "orderbook_snapshot" => {
                let inner: orderbook_updates::OrderbookSnapshot = serde_json::from_str(&s)?; 
                SocketMessage::OrderbookSnapshot(inner)
            },
            "orderbook_delta" => {
                let inner: orderbook_updates::OrderbookDelta = serde_json::from_str(&s)?; 
                SocketMessage::OrderbookDelta(inner)
            },
            _ => return Err("unrecognized message type".into())
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
        },
        _ => return None,
    };

    match msg_type_value {
        serde_json::Value::String(s) => return Some(s),
        _ => return None,
    }


}