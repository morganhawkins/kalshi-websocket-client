use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ExchangeAnnoucementsResponse{
    pub announcements: Vec<ExchangeAnnoucement>,
}

#[derive(Deserialize, Debug)]
pub struct ExchangeAnnoucement{
    pub r#type: String, // one of "info", "warning", "error"
    pub message: String,
    pub delivery_time: String, // utc date time
    pub status: String, // one of "active", "inactive"
}