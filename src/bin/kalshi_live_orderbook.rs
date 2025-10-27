use kalshi_orderbook::orderbook::connected_orderbook::ConnectedOrderbook;
use std::{env, fs};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let ticker = &args[1];

    let pub_key = fs::read_to_string("keys/kalshi-key-pub.pem").unwrap();
    let priv_key = fs::read_to_string("keys/kalshi-key.pem").unwrap();

    let conn_book = ConnectedOrderbook::new(
        ticker.as_str(), 
        pub_key.as_str(), 
        priv_key.as_str()
    ).unwrap();

    conn_book.listen().unwrap();

    loop {
        println!("\n\n\n\n\n\n\n\n\n\n\n\n\n");
        println!("{:?}", conn_book);
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    }
}
