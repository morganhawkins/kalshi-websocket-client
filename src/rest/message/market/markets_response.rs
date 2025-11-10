use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MarketsResponse {
    pub markets: Vec<Market>,
    pub cursor: String,
}

#[derive(Deserialize, Debug)]
pub struct Market {
    pub ticker: Option<String>,
    pub event_ticker: Option<String>,
    pub market_type: Option<String>,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub yes_sub_title: Option<String>,
    pub no_sub_title: Option<String>,
    pub open_time: Option<String>,
    pub close_time: Option<String>,
    pub expected_expiration_time: Option<String>,
    pub expiration_time: Option<String>,
    pub latest_expiration_time: Option<String>,
    pub settlement_timer_seconds: Option<i32>,
    pub status: Option<String>,
    pub response_price_units: Option<String>,
    pub yes_bid: Option<i32>,
    pub yes_bid_dollars: Option<String>,
    pub yes_ask: Option<i32>,
    pub yes_ask_dollars: Option<String>,
    pub no_bid: Option<i32>,
    pub no_bid_dollars: Option<String>,
    pub no_ask: Option<i32>,
    pub no_ask_dollars: Option<String>,
    pub last_price: Option<i32>,
    pub last_price_dollars: Option<String>,
    pub volume: Option<i32>,
    pub volume_24h: Option<i32>,
    pub result: Option<String>,
    pub can_close_early: Option<bool>,
    pub open_interest: Option<i32>,
    pub notional_value: Option<i32>,
    pub notional_value_dollars: Option<String>,
    pub previous_yes_bid: Option<i32>,
    pub previous_yes_bid_dollars: Option<String>,
    pub previous_yes_ask: Option<i32>,
    pub previous_yes_ask_dollars: Option<String>,
    pub previous_price: Option<i32>,
    pub previous_price_dollars: Option<String>,
    pub liquidity: Option<i32>,
    pub liquidity_dollars: Option<String>,
    pub settlement_value: Option<i32>,
    pub settlement_value_dollars: Option<String>,
    pub expiration_value: Option<String>,
    pub category: Option<String>,
    pub risk_limit_cents: Option<i32>,
    pub fee_waiver_expiration_time: Option<String>,
    pub early_close_condition: Option<String>,
    pub tick_size: Option<i32>,
    pub strike_type: Option<String>,
    pub floor_strike: Option<f32>,
    pub cap_strike: Option<String>,
    pub functional_strike: Option<String>,
    pub custom_strike: Option<String>,
    pub rules_primary: Option<String>,
    pub rules_secondary: Option<String>,
    pub mve_collection_ticker: Option<String>,
    pub mve_selected_legs: Option<Vec<MveSelectedLegs>>,
    pub primary_participant_key: Option<String>,
    pub price_level_structure: Option<String>,
    pub price_ranges: Option<Vec<PriceRanges>>,
}

#[derive(Deserialize, Debug)]
pub struct MveSelectedLegs {
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
