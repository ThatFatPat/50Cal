use fiftycal::{process_line, Context};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut ctx = Context::new();
    let mut flag = true;
    while flag {
        flag = process_line(&mut ctx)?;
    }
    Ok(())
}
