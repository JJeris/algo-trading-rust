use crate::BidOrAsk;

#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct Order {
    pub size: f64,
    pub bid_or_ask: BidOrAsk 
}