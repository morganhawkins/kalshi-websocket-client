use kalshi_orderbook::websocket::{
    client::Environment, client::KalshiWebsocketClient, message::KalshiSocketMessage,
};
use openssl::pkey::PKey;
use std::io::Write;
use std::{env, fs};

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

    let client = KalshiWebsocketClient::new(Environment::Prod);
    client.connect(pub_key.as_str(), priv_key).await.unwrap();
    client.subscribe(ticker, channel).await.unwrap();

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
                KalshiSocketMessage::TradeUpdate(trade) => {
                    let serialized_trade = serde_json::to_string(&trade);
                    if let Ok(string_struct) = serialized_trade {
                        let write_string = format!("{string_struct}\n");
                        record_file.write_all(write_string.as_bytes()).unwrap();
                    }
                }
                _ => println!("{message:?}"),
            },
            Err(e) => {
                println!("{e:?}");
            }
        }
    }
}
