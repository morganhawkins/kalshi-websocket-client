use std::{env, fs};

use kalshi_orderbook::rest::client::RestClient;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let ticker = &args[1];

    let pub_key = fs::read_to_string("keys/kalshi-key-pub.pem").unwrap();
    let priv_key = fs::read_to_string("keys/kalshi-key.pem").unwrap();

    let uri = "https://api.elections.kalshi.com";
    let path = "/trade-api/v2/markets";
    let params = vec![
        ("limit", "1"),
        ("series_ticker", "KXBTCD"),
        ("status", "open"),
    ];

    let client = RestClient::new(uri, pub_key, priv_key).unwrap();
    let response = client.get_request(path, params, "").await.unwrap();
    let text = response.text().await.unwrap();
    let json_format = serde_json::Value::from(text);
    
    println!("{:?}", json_format);

}
