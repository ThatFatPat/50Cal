use std::{
    env,
    error::Error,
    io::{self, Write},
    process,
};
fn main() -> Result<(), Box<dyn Error>> {
    loop {
        print!("{} $ ", env::current_dir()?.to_str().unwrap());
        io::stdout().flush()?;
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
        match spawn(command.args(cmdline)) {
            Ok(_status) => {}
            Err(x) => eprintln!(
                "Failed to execute command: `{}` with error: {}",
                &command_name, x
            ),
        }
    }
}

fn spawn(command: &mut process::Command) -> Result<process::ExitStatus, Box<dyn Error>> {
    Ok(command.spawn()?.wait()?)
}
