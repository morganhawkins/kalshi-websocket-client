pub mod orderbook_updates;
pub mod public_trades;

pub enum Side {
    Buy,
    Sell,
}

pub enum SocketMessage {
    OrderbookSnapshot(orderbook_updates::OrderbookSnapshot),
    OrderbookDelta(orderbook_updates::OrderbookDelta),
    TradeUpdate(public_trades::TradeUpdate),
}