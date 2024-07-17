use crate::{BidOrAsk, Limit, Order, Price};

/// Basic test of the projects functionality.
pub fn test() -> Result<(), String> {
    // Create limit.
    let mut limit = match Limit::new(65.3) {
        Ok(val) => {val},
        Err(err) => {
            return Err(format!("{}", err));
        }
    };
    // Create buy order (BID).
    let buy_order = match Order::new(5.5, BidOrAsk::BID) {
        Ok(val) => {val},
        Err(err) => {
            return Err(format!("{}", err));
        }
    };
    // Create sell order (ASK).
    let sell_order = match Order::new(2.45, BidOrAsk::ASK) {
        Ok(val) => {val},
        Err(err) => {
            return Err(format!("{}", err));
        }
    };
    // Add order to the limit.
    match limit.add_order(buy_order) {
        Ok(_) => {},
        Err(err) => {
            return Err(format!("{}", err));
        }
    };
    match limit.add_order(sell_order) {
        Ok(_) => {},
        Err(err) => {
            return Err(format!("{}", err));
        }
    };
    println!("{:#?}", limit);
    Ok(())
}