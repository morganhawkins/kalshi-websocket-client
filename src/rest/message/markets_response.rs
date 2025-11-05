use serde::Deserialize;

#[derive(Deserialize)]
pub struct MarketsResponse {
    pub market: Vec<Market>,
    pub cursor: String,
}

#[derive(Deserialize)]
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
    pub settlement_timer_seconds: String,
    pub status: String,
    pub response_price_units: String,
    pub yes_bid: String,
    pub yes_bid_dollars: String,
    pub yes_ask: String,
    pub yes_ask_dollars: String,
    pub no_bid: String,
    pub no_bid_dollars: String,
    pub no_ask: String,
    pub no_ask_dollars: String,
    pub last_price: String,
    pub last_price_dollars: String,
    pub volume: String,
    pub volume_24h: String,
    pub result: String,
    pub can_close_early: String,
    pub open_interest: String,
    pub notional_value: String,
    pub notional_value_dollars: String,
    pub previous_yes_bid: String,
    pub previous_yes_bid_dollars: String,
    pub previous_yes_ask: String,
    pub previous_yes_ask_dollars: String,
    pub previous_price: String,
    pub previous_price_dollars: String,
    pub liquidity: String,
    pub liquidity_dollars: String,
    pub settlement_value: String,
    pub settlement_value_dollars: String,
    pub expiration_value: String,
    pub category: String,
    pub risk_limit_cents: String,
    pub fee_waiver_expiration_time: String,
    pub early_close_condition: String,
    pub tick_size: String,
    pub strike_type: String,
    pub floor_strike: String,
    pub cap_strike: String,
    pub functional_strike: String,
    pub custom_strike: String,
    pub rules_primary: String,
    pub rules_secondary: String,
    pub mve_collection_ticker: String,
    pub mve_selected_legs: Vec<MveSelectedLegs>,
    pub primary_participant_key: String,
    pub price_level_structure: String,
    pub price_ranges: Vec<PriceRanges>,
}

#[derive(Deserialize)]
pub struct MveSelectedLegs{
    pub event_ticker: String,
    pub market_ticker: String,
    pub side: String,
}

#[derive(Deserialize)]
pub struct PriceRanges {
    pub start: String,
    pub end: String,
    pub step: String,
}
