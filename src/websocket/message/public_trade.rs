use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TradeUpdate {
    pub r#type: String,
    pub sid: u64,
    pub seq: u64,
    pub msg: TradeUpdateMessage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TradeUpdateMessage {
    pub trade_id: String,
    pub market_ticker: String,
    pub yes_price: u8,
    pub no_price: u8,
    pub count: u64,
    pub taker_side: String,
    pub ts: u64,
}
