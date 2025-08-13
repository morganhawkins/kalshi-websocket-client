use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

use super::SocketMessage;

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

pub struct KalshiWebsocketClient {
    uri: &'static str,
    sender: Mutex<Option<stream::SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    receiver: Mutex<Option<stream::SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
    cmd_id: Mutex<u64>,
}

impl KalshiWebsocketClient {
    pub fn new(uri: &'static str) -> Self {
        KalshiWebsocketClient {
            uri: uri,
            sender: Mutex::new(None),
            receiver: Mutex::new(None),
            cmd_id: Mutex::new(1_u64),
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
    pub async fn next_message(&self) -> Option<Result<SocketMessage, Box<dyn Error>>> {
        let mut lock = self.receiver.lock().await;
        let next = lock.as_mut().unwrap().next().await?;
        match next {
            Err(e) => Some(Err(e.into())),
            Ok(msg) => {
                let socket_message = SocketMessage::from_message(msg);
                return Some(socket_message);
            }
        }
    }

    fn create_request(
        &self,
        signer: &mut Signer,
        method: &'static str,
        path: &'static str,
        pub_key: String,
    ) -> Result<ClientRequestBuilder, Box<dyn Error>> {
        // creating current timestamp for signing
        let timestamp_num = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
        let timestamp = format!("{timestamp_num}");
        // concat info to create doc to sign
        let message = format!("{timestamp}{method}{path}");
        // sign doc and format as base64 string
        signer.update(message.as_bytes())?;
        let signature = signer.sign_to_vec()?;
        let encoded_signature = general_purpose::STANDARD.encode(&signature);
        // build upgrade request and return
        Ok(ClientRequestBuilder::new(http::Uri::from_static(self.uri))
            .with_header("KALSHI-ACCESS-KEY", pub_key)
            .with_header("KALSHI-ACCESS-SIGNATURE", encoded_signature)
            .with_header("KALSHI-ACCESS-TIMESTAMP", timestamp))
    }

    pub async fn connect(
        &self,
        pub_key: String,
        priv_key: PKey<Private>,
    ) -> Result<(), Box<dyn Error>> {
        // create signer (should have to only do this once so we drop at end of method)
        let mut signer = Signer::new(MessageDigest::sha256(), &priv_key)?;
        signer.set_rsa_padding(Padding::PKCS1_PSS)?;
        signer.set_rsa_mgf1_md(MessageDigest::sha256())?;
        signer.set_rsa_pss_saltlen(RsaPssSaltlen::DIGEST_LENGTH)?;
        // build request to authorize upgrade
        let request = self.create_request(&mut signer, "GET", "/trade-api/ws/v2", pub_key)?;
        // send connection upgrade request
        let (ws_stream, response) = connect_async(request).await?;
        if let http::StatusCode::SWITCHING_PROTOCOLS = response.status() {
            // if successful, assign sender and reciever
            println!("Authorized Websocket Connection");
            println!("Response: {response:?}");
            // split into sender reciever components and assign fields
            let (sender, receiver) = ws_stream.split();
            self.set_sender(sender).await;
            self.set_receiver(receiver).await;
        } else {
            // log failure and return Err Result
            println!("Failed to Authorize Websocket Connection");
            println!("Response: {response:?}");
            return Err(format!("failed with status code: {:?}", response.status()).into());
        };

        return Ok(());
    }

    // TODO: make channel and enum
    pub async fn subscribe(
        &self,
        market_ticker: &str,
        channel: &str,
    ) -> Result<(), Box<dyn Error>> {
        // grab next comand message id
        let id = self.get_cmd_id().await;
        let command_string = create_command(id, "subscribe", channel, market_ticker);
        // send message and await response
        self.send_message(command_string).await?;

        Ok(())
    }

    pub async fn unsubscribe(&self, _sid: &str) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

fn create_command(id: u64, cmd: &str, channel: &str, market_ticker: &str) -> String {
    format!(
        "
        {{
            \"id\": {id},
            \"cmd\": \"{cmd}\",
            \"params\": {{
                \"channels\": [\"{channel}\"],
                \"market_ticker\": \"{market_ticker}\"
            }}
        }}
        "
    )
}
