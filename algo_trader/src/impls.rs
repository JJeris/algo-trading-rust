use crate::{structs::Price, BidOrAsk, Order};

impl Order {
    /// Creates a new order instance.
    pub fn new(size: f64, bid_or_ask: BidOrAsk) -> Result<Self, String> {
        Ok(Self { size, bid_or_ask })
    }
}

impl Price {
	/// Creates a new Price instance.
    pub fn new(price: f64) -> Result<Self, String> {
        let scalar = 100000;
        let integral = price as u64;
        let fractional = ((price % 1.0) * scalar as f64) as u64;
        Ok(Self {
			integral,
			fractional,
			scalar,
		})
    }
}
