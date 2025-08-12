use crate::channels::Side;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TradeUpdate {
    pub r#type: String,
    pub sid: u64,
    pub seq: u64,
    pub msg: TradeUpdateMessage
}

#[derive(Deserialize, Debug)]
pub struct TradeUpdateMessage {
    trade_id: String,
    market_ticker: String,
    yes_price: u8,
    no_price: u8,
    count: u64,
    taker_side: String,
    ts: u64,
}


