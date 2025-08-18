use kalshi_orderbook::kalshi_channels::client::KalshiWebsocketClient;
use openssl::pkey::PKey;
use std::{fs, env};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let ticker = &args[1];
    let channel = &args[2];

    let pub_key = fs::read_to_string("keys/kalshi-key-pub.pem").unwrap();
    let priv_key_string = fs::read_to_string("keys/kalshi-key.pem").unwrap();
    let priv_key = PKey::private_key_from_pem(priv_key_string.as_bytes()).unwrap();

    let uri = "wss://api.elections.kalshi.com/trade-api/ws/v2";
    let client = KalshiWebsocketClient::new(uri);
    client.connect(pub_key, priv_key).await.unwrap();
    client
        .subscribe(ticker, channel)
        .await
        .unwrap();

    while let Some(message_result) = client.next_message().await {
        match message_result {
            Ok(message) => {
                    println!("{message:?}");
            },
            Err(e) => {
                println!("{e:?}");
            }
        }
    }
}

