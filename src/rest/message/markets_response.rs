use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MarketsResponse {
    pub markets: Vec<Market>,
    pub cursor: String,
}

#[derive(Deserialize, Debug)]
pub struct Market{
    pub ticker: String,
    pub event_ticker: String,
    pub market_type: String,
    pub title: String,
    pub subtitle: String,
    pub yes_sub_title: String,
    pub no_sub_title: String,
    pub open_time: String,
    pub close_time: String,
    pub expected_expiration_time: String,
    pub expiration_time: String,
    pub latest_expiration_time: String,
    pub settlement_timer_seconds: i32,
    pub status: String,
    pub response_price_units: String,
    pub yes_bid: i32,
    pub yes_bid_dollars: String,
    pub yes_ask: i32,
    pub yes_ask_dollars: String,
    pub no_bid: i32,
    pub no_bid_dollars: String,
    pub no_ask: i32,
    pub no_ask_dollars: String,
    pub last_price: i32,
    pub last_price_dollars: String,
    pub volume: i32,
    pub volume_24h: i32,
    pub result: String,
    pub can_close_early: bool,
    pub open_interest: i32,
    pub notional_value: i32,
    pub notional_value_dollars: String,
    pub previous_yes_bid: i32,
    pub previous_yes_bid_dollars: String,
    pub previous_yes_ask: i32,
    pub previous_yes_ask_dollars: String,
    pub previous_price: i32,
    pub previous_price_dollars: String,
    pub liquidity: i32,
    pub liquidity_dollars: String,
    pub settlement_value: i32,
    pub settlement_value_dollars: String,
    pub expiration_value: String,
    pub category: String,
    pub risk_limit_cents: i32,
    pub fee_waiver_expiration_time: Option<String>,
    pub early_close_condition: Option<String>,
    pub tick_size: i32,
    pub strike_type: String,
    pub floor_strike: f32,
    pub cap_strike: Option<String>,
    pub functional_strike: Option<String>,
    pub custom_strike: Option<String>,
    pub rules_primary: String,
    pub rules_secondary: String,
    pub mve_collection_ticker: Option<String>,
    pub mve_selected_legs: Option<Vec<MveSelectedLegs>>,
    pub primary_participant_key: Option<String>,
    pub price_level_structure: String,
    pub price_ranges: Vec<PriceRanges>,
}

#[derive(Deserialize, Debug)]
pub struct MveSelectedLegs{
    pub event_ticker: String,
    pub market_ticker: String,
    pub side: String,
}

#[derive(Deserialize, Debug)]
pub struct PriceRanges {
    pub start: String,
    pub end: String,
    pub step: String,
}
