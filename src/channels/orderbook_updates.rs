use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct OrderbookSnapshot {
    pub r#type: String,
    pub sid: u64,
    pub seq: u64,
    pub msg: OrderbookSnapshotMessage,
}

#[derive(Deserialize, Debug)]
pub struct OrderbookSnapshotMessage {
    pub market_ticker: String,
    pub market_id: String,
    pub yes: Vec<(u8, u64)>,
    pub no: Vec<(u8, u64)>,
}

// {"market_ticker":"KXBTCD-25AUG1218-T120249.99","market_id":"7b08200f-0015-4bd0-94e1-2932826fa6c4","yes":[[3,80000],[4,8100],[5,20000],[18,1000],[19,1800],[20,503],[23,200]],"no":[[1,73590],[3,80000],[4,4431],[6,5400],[7,20000],[34,8000],[40,200],[66,200],[69,2250],[70,200],[73,800],[74,3]]}}

#[derive(Deserialize, Debug)]
pub struct OrderbookDelta {
    pub r#type: String,
    pub sid: u64,
    pub seq: u64,
    pub msg: OrderbookDeltaMessage,
}

#[derive(Deserialize, Debug)]
pub struct OrderbookDeltaMessage {
    pub market_ticker: String,
    pub market_id: String,
    pub price: u8,
    pub delta: u64,
    pub side: String,
    pub ts: String,
}

// {"type":"orderbook_delta","sid":1,"seq":3,"msg":{"market_ticker":"KXMLBGAME-25AUG12SDSF-SD","market_id":"dda543e6-bd0c-4275-8814-a17f6ab61762","price":51,"delta":-1,"side":"no","ts":"2025-08-12T21:00:32.539836022Z"}
