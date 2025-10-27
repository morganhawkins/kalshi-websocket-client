use kalshi_orderbook::websocket::client::{Environment, KalshiWebsocketClient};
use openssl::pkey::PKey;
use std::{env, fs};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let ticker = &args[1];
    let channel = &args[2];

    let pub_key = fs::read_to_string("keys/kalshi-key-pub.pem").unwrap();
    let priv_key_string = fs::read_to_string("keys/kalshi-key.pem").unwrap();
    let priv_key = PKey::private_key_from_pem(priv_key_string.as_bytes()).unwrap();

    let client = KalshiWebsocketClient::new(Environment::Prod);
    client.connect(pub_key.as_str(), priv_key).await.unwrap();
    client.subscribe(ticker, channel).await.unwrap();

    while let Some(message_result) = client.next_message().await {
        match message_result {
            Ok(message) => {
                println!("{message:?}");
            }
            Err(e) => {
                println!("{e:?}");
            }
        }
    }
}
