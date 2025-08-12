use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};
use std::cell::RefCell;

use base64::{Engine as _, engine::general_purpose};
use openssl::hash::MessageDigest;
use openssl::pkey::{PKey, Private, Public};
use openssl::rsa::Padding;
use openssl::sign::{RsaPssSaltlen, Signer};
use tokio_tungstenite::tungstenite::ClientRequestBuilder;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tokio::net::TcpStream;
use futures_util::{StreamExt, stream};

struct KalshiWebsocketClient {
    uri: &'static str,
    markets: Vec<&'static str>,
    sender: RefCell<Option<stream::SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    receiver: RefCell<Option<stream::SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
}

impl KalshiWebsocketClient {
    pub fn new(uri: &'static str) -> Self {
        KalshiWebsocketClient {
            uri: uri, // TODO: make this an environment Enum for demo/prod
            markets: Vec::new(), // list of markets listening to
            sender: RefCell::new(None), 
            receiver: RefCell::new(None)
        }
    }

    fn create_request(
        &self,
        signer: &mut Signer,
        method: &'static str,
        path: &'static str,
        pub_key: &'static str,
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
        pub_key: &'static str,
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
        if let http::StatusCode::SWITCHING_PROTOCOLS = response.status(){
            // if successful, assign sender and reciever
            println!("Authorized Websocket Connection");
            println!("Response: {response:?}");
            /// split into sender reciever components and assign fields
            let (sender, receiver) = ws_stream.split();
            *self.sender.borrow_mut() = Some(sender);
            *self.receiver.borrow_mut() = Some(receiver);
        } else {
            // log failure and return Err Result
            println!("Failed to Authorize Websocket Connection");
            println!("Response: {response:?}");
            return Err(format!("failed with status code: {:?}", response.status()).into());
        };

        return Ok(());
    }
}
