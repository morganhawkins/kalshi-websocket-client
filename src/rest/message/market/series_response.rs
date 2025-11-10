use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SeriesResponse{
  pub series: Series,


}

#[derive(Deserialize, Debug)]
pub struct Series{
    pub ticker: String,
    pub frequency: String,
    pub title: String,
    pub category: String,
    pub tags: Vec<String>,
    pub settlement_sources: Vec<SettlementSource>,
    pub contract_url: String,
    pub contract_terms_url: String,
    pub product_metadata: Option<String>,
    pub fee_type: String,
    pub fee_multiplier: i32,
    pub additional_prohibitions: Option<Vec<String>>,

}

#[derive(Deserialize, Debug)]
pub struct SettlementSource{
    pub name: String,
    pub url: String,
}