use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;

use openssl::pkey::PKey;

use super::orderbook::KalshiOrderbook;
use crate::websocket::client::{Environment, KalshiWebsocketClient};
use crate::websocket::message::KalshiSocketMessage;

pub struct ConnectedOrderbook {
    book: Arc<Mutex<KalshiOrderbook>>,
    pub_key: String,
    priv_key: String,
    pub ticker: String,
}

impl ConnectedOrderbook {
    pub fn new(ticker: &str, pub_key: &str, priv_key: &str) -> Result<Self, Box<dyn Error>> {
        // construct private key object from private key String
        let book = Arc::new(Mutex::new(KalshiOrderbook::new()));
        Ok(ConnectedOrderbook {
            book: book,
            pub_key: pub_key.to_string(),
            priv_key: priv_key.to_string(),
            ticker: ticker.to_string(),
        })
    }

    pub fn listen(
        &self,
    ) -> Result<tokio::task::JoinHandle<Result<(), Box<dyn Error + Send + Sync>>>, Box<dyn Error>>
    {
        let priv_key = self.priv_key.clone();
        let pub_key = self.pub_key.clone();
        let ticker = self.ticker.clone();
        let book_clone = self.book.clone();

        let handle: tokio::task::JoinHandle<Result<(), Box<dyn Error + Send + Sync>>> =
            tokio::spawn(Self::background_update(
                pub_key, priv_key, ticker, book_clone,
            ));

        return Ok(handle);
    }

    async fn background_update(
        pub_key: String,
        priv_key: String,
        ticker: String,
        book: Arc<Mutex<KalshiOrderbook>>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let priv_key = PKey::private_key_from_pem(priv_key.as_bytes())?;
        let client = KalshiWebsocketClient::new(Environment::Prod);
        client.connect(pub_key.as_str(), priv_key).await?;
        client.subscribe(ticker.as_str(), "orderbook_delta").await?;

        while let Some(Ok(message)) = client.next_message().await {
            match message {
                KalshiSocketMessage::OrderbookSnapshot(snapshot) => {
                    let mut lock = book.lock().await;
                    lock.set_snapshot(snapshot);
                }
                KalshiSocketMessage::OrderbookDelta(delta) => {
                    let mut lock = book.lock().await;
                    lock.digest_message(delta);
                }
                other => {
                    log::info!("socket message not delta or snapshot {other:?}");
                }
            }
        }

        return Ok(());
    }

    pub fn print_book(&self) {}
}

impl std::fmt::Debug for ConnectedOrderbook {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        println!("{:?}", self.book);
        return Ok(());
    }
}
