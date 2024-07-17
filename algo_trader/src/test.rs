use crate::{Order, Limit, Price};

/// Basic test of the projects functionality.
pub fn test() -> Result<(), String> {
    let limit = match Limit::new(50.53) {
        Ok(val) => {val},
        Err(err) => {
            return Err(format!("{}", err));
        }
    };

    println!("{:#?}", limit);

    Ok(())
}