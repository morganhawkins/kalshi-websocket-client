use std::cell::RefCell;
use std::error::Error;

use reqwest::Response;
use serde::Deserialize;

use crate::rest::client::RestClient;

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

impl RestClient<'_> {
    /// Gets all markets that all under the series and event tickers specified
    ///
    /// # Arguements
    /// * series_ticker - ticker of the series to grab markets for
    /// * event_ticker - ticker of the event to grab markets for
    /// * market_tickers - tickers of the markets to grab (comma-seperated)
    /// * page_size - number of markets to grab per api request
    /// * status - status of the market. One of "open", "closed", "settled"
    /// * mve_filter - filter MVE's. One of "only", "exclude"
    ///
    /// # Examples
    /// * series ticker - "KXBTCD"
    /// * event_ticker - "KXBTCD-25NOV0513"
    /// * market_tickers - "KXBTCD-25NOV0513-T103499.99, KXBTCD-25NOV0513-T103249.99"
    /// * page_size - "100"
    /// * status - "open"
    /// * mve_filter - "only"
    ///
    /// # Returns
    /// A MarketResponse object containing all relevant markets
    ///
    ///
    pub async fn get_markets(
        &self,
        series_ticker: Option<&str>,
        event_ticker: Option<&str>,
        market_tickers: Option<&str>,
        page_size: Option<&str>,
        status: Option<&str>,
        mve_filter: Option<&str>,
    ) -> Result<MarketsResponse, Box<dyn Error>> {
        let mut base_params = Vec::new();
        Self::append_if_some(&mut base_params, "series_ticker", series_ticker);
        Self::append_if_some(&mut base_params, "event_ticker", event_ticker);
        Self::append_if_some(&mut base_params, "tickers", market_tickers);
        Self::append_if_some(&mut base_params, "limit", page_size);
        Self::append_if_some(&mut base_params, "status", status);
        Self::append_if_some(&mut base_params, "mve_filter", mve_filter);

        let mut markets = Vec::new();
        let cursor = RefCell::new(String::from(""));
        let mut next_markets_response: MarketsResponse;
        let mut text: String;
        let mut response: Response;

        loop {
            // create params with updated cursor
            let mut params = base_params.clone();
            let cursor_clone = cursor.borrow().clone();
            params.push(("cursor", &cursor_clone));

            // grabbing page of markets
            response = self
                .get_request("/trade-api/v2/markets", &params, "")
                .await?;

            // parsing text into objects
            text = response.text().await?;
            next_markets_response = serde_json::from_str(&text)?;

            // extend list of markets, update cursor
            markets.extend(next_markets_response.markets);
            *cursor.borrow_mut() = next_markets_response.cursor;

            // breaking loop if cursor is empty
            if cursor.borrow().is_empty() {
                break;
            }
        }

        Ok(MarketsResponse {
            markets: markets,
            cursor: cursor.borrow().clone(),
        })
    }
}
