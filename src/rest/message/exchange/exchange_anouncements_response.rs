use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ExchangeAnnoucementsResponse{
    pub announcements: Vec<ExchangeAnnoucement>,
}

#[derive(Deserialize, Debug)]
pub struct ExchangeAnnoucement{
    pub r#type: Option<String>, // one of "info", "warning", "error"
    pub message: Option<String>,
    pub delivery_time: Option<String>, // utc date time
    pub status: Option<String>, // one of "active", "inactive"
}