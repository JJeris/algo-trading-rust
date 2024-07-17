mod structs;
mod impls;
mod enums;

pub use structs::Order;
use structs::Price;
pub use enums::BidOrAsk;

fn main() -> Result<(), String> {
    match test() {
        Ok(_) => {Ok(())},
        Err(err) => {
            return Err(format!("{}", err));
        }
    }
}


fn test() -> Result<(), String> {
    let price = match Price::new(50.53) {
        Ok(val) => val,
        Err(err) => {
            return Err(format!("{}", err));
        }
    };

    println!("{:#?}", price);
    
    Ok(())
}
