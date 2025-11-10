use std::error::Error;

use serde::Deserialize;

use crate::rest::client::RestClient;

#[derive(Deserialize, Debug)]
pub struct ExchangeAnnoucementsResponse {
    pub announcements: Vec<ExchangeAnnoucement>,
}

#[derive(Deserialize, Debug)]
pub struct ExchangeAnnoucement {
    pub r#type: Option<String>, // one of "info", "warning", "error"
    pub message: Option<String>,
    pub delivery_time: Option<String>, // utc date time
    pub status: Option<String>,        // one of "active", "inactive"
}

impl RestClient<'_> {
    pub async fn get_exchange_announcements(
        &self,
    ) -> Result<ExchangeAnnoucementsResponse, Box<dyn Error>> {
        let response = self
            .get_request(
                "/trade-api/v2/exchange/announcements",
                Vec::new().as_ref(),
                "",
            )
            .await?;

        // parsing response text into objects
        let text = response.text().await?;
        let exchange_anouncements: ExchangeAnnoucementsResponse = serde_json::from_str(&text)?;

        return Ok(exchange_anouncements);
    }
}
