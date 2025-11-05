use std::cell::RefCell;
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

use base64::{Engine as _, engine::general_purpose};
use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::rsa::Padding;
use openssl::sign::{RsaPssSaltlen, Signer};
use reqwest::{self, RequestBuilder, Response};

use crate::rest::message;

pub struct RestClient<'a> {
    uri: String,
    signer: RefCell<Signer<'a>>,
    pub_key: String,
    client: reqwest::Client,
}

impl RestClient<'_> {
    pub fn new(
        base_uri: impl Into<String>,
        pub_key: String,
        priv_key: String,
    ) -> Result<Self, Box<dyn Error>> {
        // config request signer for auth'ed requests
        let pub_key = pub_key.as_str();
        let priv_key = PKey::private_key_from_pem(priv_key.as_bytes())?;
        let mut signer = Signer::new(MessageDigest::sha256(), &priv_key)?;
        signer.set_rsa_padding(Padding::PKCS1_PSS)?;
        signer.set_rsa_mgf1_md(MessageDigest::sha256())?;
        signer.set_rsa_pss_saltlen(RsaPssSaltlen::DIGEST_LENGTH)?;
        // base requests

        Ok(Self {
            uri: base_uri.into(),
            signer: RefCell::new(signer),
            pub_key: pub_key.to_string(),
            client: reqwest::Client::new(),
        })
    }
    
    fn base_get_request(&self, path: &str) -> Result<RequestBuilder, Box<dyn Error>> {
        let timestamp_num = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
        let timestamp = format!("{timestamp_num}");
        // concat info to create doc to sign
        let message = format!("{timestamp}GET{path}");
        // sign doc and format as base64 string
        self.signer.borrow_mut().update(message.as_bytes())?;
        let signature = self.signer.borrow().sign_to_vec()?;
        let encoded_signature = general_purpose::STANDARD.encode(&signature);
        // concat base endpoint and path
        let endpoint = self.uri.clone() + path;
        
        let base_req = self
        .client
        .get(endpoint)
        .header("Content-Type", "application/json")
        .header("KALSHI-ACCESS-KEY", self.pub_key.clone())
        .header("KALSHI-ACCESS-SIGNATURE", encoded_signature)
        .header("KALSHI-ACCESS-TIMESTAMP", timestamp);
    
        Ok(base_req)
    }

    fn base_post_request(&self, path: &str) -> Result<RequestBuilder, Box<dyn Error>> {
        let timestamp_num = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
        let timestamp = format!("{timestamp_num}");
        // concat info to create doc to sign
        let message = format!("{timestamp}POST{path}");
        // sign doc and format as base64 string
        self.signer.borrow_mut().update(message.as_bytes())?;
        let signature = self.signer.borrow().sign_to_vec()?;
        let encoded_signature = general_purpose::STANDARD.encode(&signature);
        // concat base endpoint and path
        let endpoint = self.uri.clone() + path;
        
        let base_req = self
        .client
        .post(endpoint)
        .header("Content-Type", "application/json")
        .header("KALSHI-ACCESS-KEY", self.pub_key.clone())
        .header("KALSHI-ACCESS-SIGNATURE", encoded_signature)
        .header("KALSHI-ACCESS-TIMESTAMP", timestamp);

        Ok(base_req)
    }

    pub async fn get_request(
        &self,
        path: &str,
        params: Vec<(&str, &str)>,
        body: impl Into<String>,
    ) -> Result<Response, Box<dyn Error>> {
        // format & send request
        let response = self
            .base_get_request(path)?
            .query(&params)
            .body(body.into())
            .send()
            .await?;

        Ok(response)
    }

    fn append_if_some <'a, 'b> (
        v: &'a mut Vec<(&'b str, &'b str)>, 
        arg_key: &'b str,
        arg_value_option: Option<&'b str>,
    ) 
    where 'b: 'a
    {
        if let Some(arg_value) = arg_value_option {
            v.push((arg_key, arg_value))
        }
    }

    pub async fn get_markets(
        &self,
        series_ticker: Option<&str>,
        event_ticker: Option<&str>,
        page_size: Option<&str>,
        status: Option<&str>,
        mve_filter: Option<&str>,
    ) -> Result<message::MarketsResponse, Box<dyn Error>> {
        let mut params = Vec::new();
        Self::append_if_some(&mut params,"series_ticker", series_ticker);
        Self::append_if_some(&mut params,"event_ticker", event_ticker);
        Self::append_if_some(&mut params,"limit", page_size);
        Self::append_if_some(&mut params,"status", status);
        Self::append_if_some(&mut params,"mve_filter", mve_filter);

        let response = self.get_request(
            "/trade-api/v2/markets", 
            params, 
            ""
        ).await?;
        let text = response.text().await?;
        let markets_response: message::MarketsResponse = serde_json::from_str(&text)?;

        Ok(markets_response)
    }

    
}





