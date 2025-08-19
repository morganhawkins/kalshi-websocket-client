use kalshi_orderbook::kalshi_channels::{client::KalshiWebsocketClient, KalshiSocketMessage};
use openssl::{pkey::PKey};
use std::{fs, env};
use std::io::Write;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let ticker = &args[1];
    let channel = &args[2];
    let write_path = &args[3];

    let mut record_file = fs::OpenOptions::new()
        .append(true)
        .open(write_path)
        .unwrap();

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
            Ok(message) => match message {
                KalshiSocketMessage::OrderbookSnapshot(snap) => {
                    let serialized_snap = serde_json::to_string(&snap);
                    if let Ok(string_struct) = serialized_snap {
                        let write_string = format!("{string_struct}\n");
                        record_file.write_all(write_string.as_bytes()).unwrap();
                    }
                }
                KalshiSocketMessage::OrderbookDelta(delta) => {
                    let serialized_delta = serde_json::to_string(&delta);
                    if let Ok(string_struct) = serialized_delta {
                        let write_string = format!("{string_struct}\n");
                        record_file.write_all(write_string.as_bytes()).unwrap();
                    }
                }
                _ => println!("{message:?}")
            },
            Err(e) => {
                println!("{e:?}");
            }
        }
    }
}

