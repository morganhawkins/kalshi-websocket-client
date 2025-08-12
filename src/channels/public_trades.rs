use crate::channels::Side;

pub struct TradeUpdate {
    pub side: u64,
    pub market_ticker: &'static str,
    pub price: u8,
    pub count: u64,
    pub taker_side: Side,
    pub ts: u64,
}
