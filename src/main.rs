use std::fs;

use openssl::pkey::PKey;
use tokio_tungstenite::tungstenite::Message;

use kalshi_orderbook::channels::client::KalshiWebsocketClient;
use kalshi_orderbook::channels::SocketMessage;

#[tokio::main]
async fn main() {
    let pub_key = fs::read_to_string("kalshi-key-pub.pem").unwrap();
    let priv_key_string = fs::read_to_string("kalshi-key.pem").unwrap();
    let priv_key = PKey::private_key_from_pem(priv_key_string.as_bytes()).unwrap();

    let uri = "wss://api.elections.kalshi.com/trade-api/ws/v2";
    let client = KalshiWebsocketClient::new(uri);
    client.connect(pub_key, priv_key).await.unwrap();
    client.subscribe("KXBTCD-25AUG1218-T120249.99", "orderbook_delta").await.unwrap();

    std::thread::sleep(std::time::Duration::from_secs(2));

    while let Some(message_result) = client.next_message().await {
        match message_result {
            Ok(Message::Text(text)) => {
                let text = text.to_string();
                println!("Text Message: {text}");
                println!("Serialized Message: {:?}", SocketMessage::from_str(text));
            }
            _ => println!("Non-Text Message: {message_result:?}"),
        }
        
    }
}