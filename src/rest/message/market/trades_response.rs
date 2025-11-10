use std::cell::RefCell;
use std::error::Error;

use reqwest::Response;
use serde::Deserialize;

use crate::rest::client::RestClient;

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

impl RestClient<'_> {
    pub async fn get_trades(
        &self,
        market_ticker: &str,
        page_size: Option<&str>,
        min_ts: Option<&str>,
        max_ts: Option<&str>,
        max_trades: Option<i32>,
    ) -> Result<TradesResponse, Box<dyn Error>> {
        let mut base_params = Vec::new();
        Self::append_if_some(&mut base_params, "limit", page_size);
        Self::append_if_some(&mut base_params, "min_ts", min_ts);
        Self::append_if_some(&mut base_params, "max_ts", max_ts);
        base_params.push(("ticker", market_ticker));

        let mut trades = Vec::new();
        let cursor = RefCell::new(String::from(""));
        let mut next_trades_response: TradesResponse;
        let mut text: String;
        let mut response: Response;

        loop {
            // create params with updated cursor
            let mut params = base_params.clone();
            let cursor_clone = cursor.borrow().clone();
            params.push(("cursor", &cursor_clone));

            // grabbing page of markets
            response = self
                .get_request("/trade-api/v2/markets/trades", &params, "")
                .await?;

            // parsing text into objects
            text = response.text().await?;
            next_trades_response = serde_json::from_str(&text)?;

            // extend list of markets, update cursor
            trades.extend(next_trades_response.trades);
            *cursor.borrow_mut() = next_trades_response.cursor;

            if cursor.borrow().is_empty() {
                // breaking loop if cursor is empty
                break;
            } else if let Some(max) = max_trades {
                // breaking loop if number of trades returned has exceeded
                if trades.len() >= max as usize {
                    break;
                }
            }
        }

        Ok(TradesResponse {
            trades: trades,
            cursor: cursor.borrow().clone(),
        })
    }
}
