use fiftycal::{process_line, prompt, Context};
use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let mut ctx = Context::new();
    loop {
        prompt(&ctx)?;
        process_line(&mut ctx)?;
    }
}
