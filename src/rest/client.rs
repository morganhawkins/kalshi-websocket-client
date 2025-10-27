use std::error::Error;

use openssl::sign::{RsaPssSaltlen, Signer};
use openssl::pkey::PKey;
use openssl::rsa::Padding;
use openssl::hash::MessageDigest;

struct RestClient<'a>{
    uri: String,
    signer: Signer<'a>,
}

impl RestClient <'_> {
    pub fn new(
        pub_key: String,
        priv_key: String,
    ) -> Result<Self, Box<dyn Error>>{
        let pub_key = pub_key.as_str();
        let priv_key = PKey::private_key_from_pem(priv_key.as_bytes())?;
        let mut signer = Signer::new(MessageDigest::sha256(), &priv_key)?;
            signer.set_rsa_padding(Padding::PKCS1_PSS)?;
            signer.set_rsa_mgf1_md(MessageDigest::sha256())?;
            signer.set_rsa_pss_saltlen(RsaPssSaltlen::DIGEST_LENGTH)?;

        Ok(Self {
            uri: "https://api.elections.kalshi.com/trade-api/v2/".to_string(),
            signer: signer,
        })

    }
}