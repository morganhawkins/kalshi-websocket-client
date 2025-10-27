use super::websocket::message::orderbook_update::OrderbookDelta;
use super::websocket::message::orderbook_update::OrderbookSnapshot;

#[derive(Debug)]
pub struct KalshiOrderbook {
    pub bid_orders: [i64; 99],
    pub ask_orders: [i64; 99],
}

impl KalshiOrderbook {
    pub fn new() -> Self {
        Self {
            bid_orders: [0_i64; 99],
            ask_orders: [0_i64; 99],
        }
    }

    pub fn from_snapshot(snapshot: OrderbookSnapshot) -> Self {
        let mut yes_book = [0i64; 99];
        let mut no_book = [0i64; 99];
        // copy values from snapshot into liquidity array
        let mut idx: usize;
        match snapshot.msg.yes {
            // if snapshot has a yes field, generate bids from this
            Some(bids) => {
                for (price, quant) in bids {
                    idx = (price as usize) - 1usize;
                    yes_book[idx] += quant as i64;
                }
            }
            None => (),
        };
        match snapshot.msg.no {
            // if snapshot has a no field, generate asks from this
            Some(asks) => {
                for (price, quant) in asks {
                    idx = (100 - price as usize) - 1usize;
                    no_book[idx] += quant as i64;
                }
            }
            None => (),
        };
        // return copied
        Self {
            bid_orders: yes_book,
            ask_orders: no_book,
        }
    }

    // change quantity at bid price
    fn delta_bid(&mut self, price: usize, quant_delta: i64) {
        self.bid_orders[price - 1usize] += quant_delta;
    }

    // change quantity at ask price
    fn delta_ask(&mut self, price: usize, quant_delta: i64) {
        self.ask_orders[price - 1usize] += quant_delta;
    }

    pub fn digest_message(&mut self, message: OrderbookDelta) {
        match message.msg.side.as_str() {
            "yes" => {
                self.delta_bid(message.msg.price as usize, message.msg.delta);
            }
            "no" => {
                self.delta_ask(100 - message.msg.price as usize, message.msg.delta);
            }
            _ => {
                panic!("orderbook delta message has side not in [yes, no]");
            }
        }
    }

    pub fn print_book(&self) {
        println!("\n\n---BOOK---");
        for (price, quant) in self.ask_orders.iter().enumerate().rev() {
            if *quant != 0 {
                println!("{}, {quant}", price + 1);
            }
        }
        println!("------");
        for (price, quant) in self.bid_orders.iter().enumerate().rev() {
            if *quant != 0 {
                println!("{}, {quant}", price + 1);
            }
        }
    }
}
