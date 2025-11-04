use std::str::FromStr;

use serde_json::Value;
use http::status::StatusCode;
use reqwest::Response;

pub struct RestResponse{
    text: Option<String>,
    status: StatusCode,
}

impl RestResponse {
    pub fn new(text: Option<String>, status: StatusCode) -> RestResponse {
        RestResponse{
            text,
            status,
        }
    }

    // TODO: surely not the right way to do this
    pub async fn from_reqwest_response(resp: Response) -> RestResponse {
        let status = resp.status();
        let text = resp.text().await.ok();
        RestResponse {
            text, 
            status,
        }
    }

    pub fn text(&self) -> Option<String>{
        self.text.clone()
    }

    pub fn status(&self) -> StatusCode{
        self.status
    }

    pub fn json(&self) -> Option<Value>{
        if let Some(text) = &self.text(){
            Value::from_str(text).ok()
        } else {
            None
        }
    }
}

