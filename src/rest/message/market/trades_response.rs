use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TradesResponse {
    pub trades: Vec<Trade>,
    pub cursor: String,
}

#[derive(Deserialize, Debug)]
pub struct Trade {
    pub trade_id: String,
    pub ticker: String,
    pub price: f32,
    pub count: i32,
    pub yes_price: u8,
    pub no_price: u8,
    pub yes_price_dollars: String,
    pub no_price_dollars: String,
    pub taker_side: String,
    pub created_time: String,
}
