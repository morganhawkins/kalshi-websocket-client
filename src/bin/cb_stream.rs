use base64::Engine;
use kalshi_orderbook::coinbase_channels::client::CoinbaseWebsocketClient;
use openssl::pkey::PKey;
use std::fs;

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
