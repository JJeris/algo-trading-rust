use std::collections::HashMap;

use crate::BidOrAsk;

#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct Order {
    pub size: f64,
    pub bid_or_ask: BidOrAsk 
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct OrderBook {
    pub asks: HashMap<Price, Limit>, // Need tp add Eq. PartialEq and Hash to the Price struct.
    pub bids: HashMap<Price, Limit>,
}


#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Price {
    pub integral: u64,
    pub fractional: u64,
    pub scalar: u64,
}


#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct Limit {
    pub price: Price, // Use a custom implementation, so that it can be safely used in a hashmap.
    pub orders: Vec<Order>,

}