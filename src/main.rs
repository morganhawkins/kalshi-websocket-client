use base64::Engine;
use kalshi_orderbook::coinbase_channels::client::CoinbaseWebsocketClient;
use kalshi_orderbook::kalshi_channels::KalshiSocketMessage;
use kalshi_orderbook::kalshi_channels::client::KalshiWebsocketClient;
use kalshi_orderbook::kalshi_orderbook::KalshiOrderbook;
use openssl::pkey::PKey;
use std::fs;

// #[tokio::main]
// async fn main() {
//     let pub_key = fs::read_to_string("keys/kalshi-key-pub.pem").unwrap();
//     let priv_key_string = fs::read_to_string("keys/kalshi-key.pem").unwrap();
//     let priv_key = PKey::private_key_from_pem(priv_key_string.as_bytes()).unwrap();

//     let uri = "wss://api.elections.kalshi.com/trade-api/ws/v2";
//     let client = KalshiWebsocketClient::new(uri);
//     client.connect(pub_key, priv_key).await.unwrap();
//     client
//         .subscribe("KXBTCD-25AUG1412-T118249.99", "orderbook_delta")
//         .await
//         .unwrap();
//     let mut book = KalshiOrderbook::new();

//     while let Some(message_result) = client.next_message().await {
//         // println!("{message_result:?}");
//         match message_result {
//             Ok(message) => match message {
//                 KalshiSocketMessage::OrderbookSnapshot(snapshot) => {
//                     println!("{snapshot:?}");
//                     book = KalshiOrderbook::from_snapshot(snapshot);
//                 }
//                 KalshiSocketMessage::OrderbookDelta(delta) => {
//                     // println!("{delta:?}");
//                     book.digest_message(delta);
//                 }
//                 _ => {
//                     println!("{message:?}");
//                 }
//             },
//             Err(e) => {
//                 println!("{e:?}");
//             }
//         }
//         book.print_book();
//     }
// }

#[tokio::main]
async fn main() {
    let pub_key = fs::read_to_string("keys/cb-key-pub.pem").unwrap();
    let priv_key_string = fs::read_to_string("keys/cb-key.pem").unwrap();
    let priv_key_bytes = base64::engine::general_purpose::STANDARD.decode(priv_key_string).unwrap();
    let priv_key =
        PKey::private_key_from_raw_bytes(&priv_key_bytes, openssl::pkey::Id::HMAC)
            .unwrap();

    let uri = "wss://ws-feed.exchange.coinbase.com";
    let client = CoinbaseWebsocketClient::new(uri);
    client.connect(pub_key, priv_key).await.unwrap();
    client.subscribe("BTC-USD").await.unwrap();

    while let Some(message_result) = client.next_message().await {
        println!("{message_result:?}\n");
    }
}
