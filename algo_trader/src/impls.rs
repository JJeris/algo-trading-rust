use crate::{BidOrAsk, Order};

impl Order {
    
    /// Creates a new order instance.
    fn new(bid_or_ask: BidOrAsk, size: f64) -> Result<Order, String> {
        Ok(Self {
            size,
            bid_or_ask,
        })
    }
}