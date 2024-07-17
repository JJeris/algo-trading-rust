use crate::structs::Price;

/// Basic test of the projects functionality.
pub fn test() -> Result<(), String> {
    let price = match Price::new(50.53) {
        Ok(val) => val,
        Err(err) => {
            return Err(format!("{}", err));
        }
    };

    println!("{:#?}", price);
    
    Ok(())
}