use std::collections::HashMap;

use crate::{BidOrAsk, Limit, Order, OrderBook, Price};


impl Order {
    /// Creates a new order instance.
    pub fn new(size: f64, bid_or_ask: BidOrAsk) -> Result<Self, String> {
        Ok(Self { size, bid_or_ask })
    }
}

impl OrderBook {
	/// Creates a new OrderBook instance.
	pub fn new() -> Result<OrderBook, String> {
		Ok(Self {
			asks: HashMap::new(),
			bids: HashMap::new(),

		})
	}

	/// Adds an order.
	pub fn add_order(&mut self, price: f64, order: Order) -> Result<(), String> {
		match order.bid_or_ask {
			BidOrAsk::BID => {
				let price = Price::new(price).unwrap();
				let limit = self.bids.get_mut(&price);

				match limit {
					Some(val) => {
						println!("Already have a limit.");
					},
					None => {
						println!("Don't have a limit.");
					},
				}
			},
			BidOrAsk::ASK => {},
		}
		Ok(())
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


impl Limit {
	/// Creates a new Limit instance.
	pub fn new(price: f64) -> Result<Self, String> {
		Ok(Self {
			price: match Price::new(price) {
				Ok(val) => {val},
				Err(err) => {
					return Err(format!("{}", err));
				}
			},
			orders: Vec::new(),
		})	
	}
	/// Adds an order to the orders field.
	pub fn add_order(&mut self, order: Order) -> Result<(), String> {
		self.orders.push(order);
		Ok(())
	}
}