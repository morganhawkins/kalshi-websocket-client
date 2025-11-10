use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SeriesResponse{
  pub series: Series,


}

#[derive(Deserialize, Debug)]
pub struct Series{
    pub ticker: String,
    pub frequency: Option<String>,
    pub title: Option<String>,
    pub category: Option<String>,
    pub tags: Vec<Option<String>>,
    pub settlement_sources: Vec<Option<SettlementSource>>,
    pub contract_url: Option<String>,
    pub contract_terms_url: Option<String>,
    pub product_metadata: Option<String>,
    pub fee_type: Option<String>,
    pub fee_multiplier: Option<i32>,
    pub additional_prohibitions: Option<Vec<String>>,

}

#[derive(Deserialize, Debug)]
pub struct SettlementSource{
    pub name: String,
    pub url: String,
}