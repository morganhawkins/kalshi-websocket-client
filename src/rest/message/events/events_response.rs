use std::cell::RefCell;
use std::error::Error;

use reqwest::Response;
use serde::Deserialize;

use crate::rest::message::market::Market;
use crate::rest::client::RestClient;

#[derive(Deserialize, Debug)]
pub struct EventsResponse{
    pub events: Vec<Event>,
    pub milestones: Vec<Milestone>,
    pub cursor: String,    
}

#[derive(Deserialize, Debug)]
pub struct Event{

      pub event_ticker: String,
      pub series_ticker: String,
      pub sub_title: String,
      pub title: String,
      pub collateral_return_type: String,
      pub mutually_exclusive: bool,
      pub category: String,
      pub strike_date: String,
      pub strike_period:String,
      pub markets: Option<Vec<Market>>,
      pub available_on_brokers: bool,
      pub product_metadata: Option<String>
}

#[derive(Deserialize, Debug)]
pub struct Milestone{
    pub id: String,
    pub category: String,
    pub r#type: String,
    pub start_date: String,
    pub end_date: String,
    pub related_event_tickers: Vec<String>,
    pub title: String,
    pub notification_message: String,
    pub source_id: String,
    pub details: Option<String>,
    pub primary_event_tickers: Vec<String>,
    pub last_updated_ts: String,    
}


impl RestClient<'_> {
    pub async fn get_events(
        &self,
        series_ticker: Option<&str>,
        page_size: Option<&str>,
        status: Option<&str>,
        with_nested_markets: Option<&str>
    ) -> Result<EventsResponse, Box<dyn Error>> {
        let mut base_params = Vec::new();
        Self::append_if_some(&mut base_params, "series_ticker", series_ticker);
        Self::append_if_some(&mut base_params, "limit", page_size);
        Self::append_if_some(&mut base_params, "status", status);
        Self::append_if_some(&mut base_params, "with_nested_markets", with_nested_markets);

        let mut events = Vec::new();
        let mut milestones = Vec::new();
        let cursor = RefCell::new(String::from(""));
        let mut next_events_response: EventsResponse;
        let mut text: String;
        let mut response: Response;

        loop {
            // create params with updated cursor
            let mut params = base_params.clone();
            let cursor_clone = cursor.borrow().clone();
            params.push(("cursor", &cursor_clone));

            // grabbing page of markets
            response = self
                .get_request("/trade-api/v2/events", &params, "")
                .await?;

            // parsing text into objects
            text = response.text().await?;
            next_events_response = serde_json::from_str(&text)?;

            // extend list of markets, update cursor
            events.extend(next_events_response.events);
            milestones.extend(next_events_response.milestones);
            *cursor.borrow_mut() = next_events_response.cursor;

            // breaking loop if cursor is empty
            if cursor.borrow().is_empty() {
                break;
            }
        }

        Ok(EventsResponse {
            events: events,
            milestones: milestones,
            cursor: cursor.borrow().clone(),
        })
    }
}