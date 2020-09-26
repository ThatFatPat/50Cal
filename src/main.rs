use std::{
    env,
    error::Error,
    io::{self, Write},
    process,
};
fn main() -> Result<(), Box<dyn Error>> {
    loop {
        print!("{} $ ", env::current_dir().unwrap().to_str().unwrap());
        io::stdout().flush().expect("flush failed");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let mut cmdline = input.split_whitespace();
        let command_name = match cmdline.next() {
            Some(x) => x,
            None => continue,
        };
        if command_name == "exit" {
            return Ok(());
        }
        let mut command = process::Command::new(&command_name);
        match command.args(cmdline).spawn() {
            Ok(mut child) => {
                child.wait().expect("Command wasn't running!");
            }
            Err(x) => {
                eprintln!(
                    "Failed to execute command: `{}` with error: {}",
                    &command_name, x
                );
            }
        }
        io::stdout().flush().expect("flush failed");
    }
}
