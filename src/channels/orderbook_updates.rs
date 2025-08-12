use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct OrderbookSnapshot {
    pub r#type: String,
    pub sid: u64,
    pub seq: u64,
    pub msg: OrderbookSnapshotMessage,
}

#[derive(Deserialize, Debug)]
pub struct OrderbookSnapshotMessage {
    pub market_ticker: String,
    pub market_id: String,
    pub yes: Vec<(u8, u64)>,
    pub no: Vec<(u8, u64)>,
}

#[derive(Deserialize, Debug)]
pub struct OrderbookDelta {
    pub r#type: String,
    pub sid: u64,
    pub seq: u64,
    pub msg: OrderbookDeltaMessage,
}

#[derive(Deserialize, Debug)]
pub struct OrderbookDeltaMessage {
    pub market_ticker: String,
    pub market_id: String,
    pub price: u8,
    pub delta: u64,
    pub side: String,
    pub ts: String,
}
