use kalshi_orderbook::channels::SocketMessage;
use kalshi_orderbook::channels::client::KalshiWebsocketClient;
use kalshi_orderbook::kalshi_orderbook::KalshiOrderbook;
use openssl::pkey::PKey;
use std::fs;

#[tokio::main]
async fn main() {
    let pub_key = fs::read_to_string("keys/kalshi-key-pub.pem").unwrap();
    let priv_key_string = fs::read_to_string("keys/kalshi-key.pem").unwrap();
    let priv_key = PKey::private_key_from_pem(priv_key_string.as_bytes()).unwrap();

    let uri = "wss://api.elections.kalshi.com/trade-api/ws/v2";
    let client = KalshiWebsocketClient::new(uri);
    client.connect(pub_key, priv_key).await.unwrap();
    client
        .subscribe("KXETHD-25AUG1412-T4599.99", "orderbook_delta")
        .await
        .unwrap();
    let mut book = KalshiOrderbook::new();

    while let Some(message_result) = client.next_message().await {
        // println!("{message_result:?}");
        match message_result {
            Ok(message) => match message {
                SocketMessage::OrderbookSnapshot(snapshot) => {
                    println!("{snapshot:?}");
                    book = KalshiOrderbook::from_snapshot(snapshot);
                }
                SocketMessage::OrderbookDelta(delta) => {
                    // println!("{delta:?}");
                    book.digest_message(delta);
                }
                _ => {
                    println!("{message:?}");
                }
            },
            Err(e) => {
                println!("{e:?}");
            }
        }
        book.print_book();
    }
}
