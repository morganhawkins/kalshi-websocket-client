use std::{env, fs};

use kalshi_orderbook::rest::client::RestClient;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let pub_key = fs::read_to_string("keys/kalshi-key-pub.pem").unwrap();
    let priv_key = fs::read_to_string("keys/kalshi-key.pem").unwrap();

    let uri = "https://api.elections.kalshi.com";
    let client = RestClient::new(uri, pub_key, priv_key).unwrap();

    let response = client.get_markets(
        Some("KXBTCD"), 
        Some("KXBTCD-25NOV0916"), 
        None,
        Some("200"), 
        Some("open"), 
        None
    ).await.unwrap();

    println!("{}", response.markets.len());
    for market in response.markets{
        println!("{:?} {:?} {:?} ", market.ticker, market.close_time,  market.liquidity_dollars)
    }

    
}
