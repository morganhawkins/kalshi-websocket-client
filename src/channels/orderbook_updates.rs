use crate::channels::Side;

pub struct OrderbookSnapshot {
    pub sid: u64,
    pub seq: u64,
    pub market_ticker: &'static str,
    pub yes: Vec<(u8, u64)>,
    pub no: Vec<(u8, u64)>,
    pub ts: u64,
}

pub struct OrderbookDelta {
    pub sid: u64,
    pub seq: u64,
    pub market_ticker: &'static str,
    pub price: u8,
    pub delta: i64,
    pub side: Side,
    pub ts: u64,
}
