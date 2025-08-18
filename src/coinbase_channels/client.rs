use core::time;
use std::error::Error;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
use std::cell::RefCell;

use super::CoinbaseSocketMessage;

use base64::{Engine as _, engine::general_purpose};
use futures_util::{SinkExt, StreamExt, stream};
use openssl::hash::MessageDigest;
use openssl::pkey::{PKey, Private};
use openssl::rsa::Padding;
use openssl::sign::{RsaPssSaltlen, Signer};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::ClientRequestBuilder;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::{connect_async, tungstenite::Message};

pub struct CoinbaseWebsocketClient {
    uri: &'static str,
    auth_uri: String,
    sender: Mutex<Option<stream::SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    receiver: Mutex<Option<stream::SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
    cmd_id: Mutex<u64>,
    priv_key: RefCell<Option<PKey<Private>>>,
    pub_key: RefCell<Option<String>>,
}

impl CoinbaseWebsocketClient {
    pub fn new(uri: &'static str) -> Self {
        CoinbaseWebsocketClient {
            uri: uri,
            auth_uri: format!("{}/users/self/verify", uri),
            sender: Mutex::new(None),
            receiver: Mutex::new(None),
            cmd_id: Mutex::new(1_u64),
            priv_key: RefCell::new(None),
            pub_key: RefCell::new(None),
        }
    }

    async fn get_cmd_id(&self) -> u64 {
        let mut lock = self.cmd_id.lock().await;
        *lock += 1;
        *lock
    }

    async fn set_sender(
        &self,
        sender: stream::SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    ) {
        let mut lock = self.sender.lock().await;
        *lock = Some(sender);
    }

    async fn set_receiver(
        &self,
        receiver: stream::SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    ) {
        let mut lock = self.receiver.lock().await;
        *lock = Some(receiver);
    }

    async fn send_message(&self, message: String) -> Result<(), Box<dyn Error>> {
        let tung_message = tokio_tungstenite::tungstenite::Message::text(message);
        let mut lock = self.sender.lock().await;
        // TODO: pattern match this and clean True
        if lock.is_some() {
            return Ok(lock.as_mut().unwrap().send(tung_message).await?);
        } else {
            return Err("`sender` field is none. call connect method first".into());
        }
    }

    // TODO: does the reader actually need to be behind Mutex??
    pub async fn next_message(&self) -> Option<Result<CoinbaseSocketMessage, Box<dyn Error>>> {
        let mut lock = self.receiver.lock().await;
        let next = lock.as_mut().unwrap().next().await?;
        match next {
            Err(e) => {
                Some(Err(e.into()))
            },
            Ok(msg) => {
                let socket_message = CoinbaseSocketMessage::from_message(msg);
                return Some(socket_message)
            }
        }
        // Some(next.map_err(|e| e.into()))
    }

    fn generate_signature(
        &self,
        signer: &mut Signer,
        method: &'static str,
        path: &'static str,
        timestamp: &str,
    ) -> Result<String, Box<dyn Error>> {
        // concat info to create doc to sign
        let message = format!("{timestamp}{method}{path}");
        // sign doc and format as base64 string
        signer.update(message.as_bytes())?;
        let signature = signer.sign_to_vec()?;
        let encoded_signature = general_purpose::STANDARD.encode(&signature);
        Ok(encoded_signature)
    }

    pub async fn connect(
        &self,
        pub_key: String,
        priv_key: PKey<Private>,
    ) -> Result<(), Box<dyn Error>> {
        // send connection upgrade request
        let request = self.uri;
        let (ws_stream, response) = connect_async(request).await?;
        if let http::StatusCode::SWITCHING_PROTOCOLS = response.status() {
            // if successful, assign sender and reciever
            println!("Authorized Coinbase Websocket Connection");
            println!("Response: {response:?}");
            // split into sender reciever components and assign fields
            let (sender, receiver) = ws_stream.split();
            self.set_sender(sender).await;
            self.set_receiver(receiver).await;
            // set key fields if successful connection
            *self.priv_key.borrow_mut() = Some(priv_key);
            *self.pub_key.borrow_mut() = Some(pub_key);
        } else {
            // log failure and return Err Result
            println!("Failed to Authorize Websocket Connection");
            println!("Response: {response:?}");
            return Err(format!("failed with status code: {:?}", response.status()).into());
        };

        return Ok(());
    }

    fn generate_subscribe_message(
        &self,
        channel: &str,
        market_ticker: &str,
    ) -> Result<String, Box<dyn Error>> {
        // grab keys from fields
        let priv_key = self.priv_key.borrow().clone();
        let pub_key = self.pub_key.borrow().clone().ok_or("Public key not set, call connect method")?;
        let priv_key_ref = match priv_key {
            Some(key) => key,
            None => return Err("Private key not set, call self.connect first".into()),
        };
        // creating current timestamp for signing
        let timestamp_num = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let timestamp = format!("{timestamp_num}");

        // create signer & signature(should have to only do this once so we drop at end of method)
        // let mut signer = Signer::new(MessageDigest::sha256(), &priv_key_ref)?;
        // signer.set_rsa_padding(Padding::PKCS1_PSS)?;
        // signer.set_rsa_mgf1_md(MessageDigest::sha256())?;
        // signer.set_rsa_pss_saltlen(RsaPssSaltlen::DIGEST_LENGTH)?;
        // let signature = self.generate_signature(&mut signer, "GET", "/users/self/verify", &timestamp)?;
        let signature = String::from("blank");
        let msg = format!("
            {{
        \"type\": \"subscribe\",
        \"product_ids\": [\"ETH-USD\", \"ETH-EUR\"],
        \"channels\": [
                \"level2\",
                \"heartbeat\",
                {{
                    \"name\": \"ticker\",
                    \"product_ids\": [\"ETH-BTC\", \"ETH-USD\"]
                }}
            ]
            }}
        ");

        Ok(msg)
    }

    // TODO: make channel and enum
    pub async fn subscribe(&self, market_ticker: &str) -> Result<(), Box<dyn Error>> {
        // generate subscribe message
        let message = self.generate_subscribe_message("level2", market_ticker)?;
        // send message and await response
        self.send_message(message).await?;
        Ok(())
    }

    pub async fn unsubscribe(&self, _sid: &str) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

