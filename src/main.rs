use fiftycal::{process_cmdline, prompt, Context};
use std::{error::Error, io};
fn main() -> Result<(), Box<dyn Error>> {
    let mut ctx = Context::new();
    loop {
        prompt()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        process_cmdline(&input, &mut ctx)?;
    }
}
