use std::fs;
use kalshi_orderbook::channels::client::KalshiWebsocketClient;
use openssl::pkey::PKey;

#[tokio::main]
async fn main() {
    let pub_key = fs::read_to_string("keys/kalshi-key-pub.pem").unwrap();
    let priv_key_string = fs::read_to_string("keys/kalshi-key.pem").unwrap();
    let priv_key = PKey::private_key_from_pem(priv_key_string.as_bytes()).unwrap();

    let uri = "wss://api.elections.kalshi.com/trade-api/ws/v2";
    let client = KalshiWebsocketClient::new(uri);
    client.connect(pub_key, priv_key).await.unwrap();
    client
        .subscribe("KXBTCD-25AUG1318-T122749.99", "orderbook_delta")
        .await
        .unwrap();

    std::thread::sleep(std::time::Duration::from_secs(2));

    while let Some(message_result) = client.next_message().await {
        println!("{message_result:?}")
    }
}
