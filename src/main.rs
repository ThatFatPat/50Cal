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
        process_cmdline(&input)?;
    }
}
fn process_cmdline(line: &str) -> Result<(), Box<dyn Error>> {
    let mut cmdline = line.split_whitespace();
    let command_name = match cmdline.next() {
        Some(x) => x,
        None => return Ok(()),
    };
    match command_name {
        "exit" => process::exit(0),
        _ => {
            let mut command = process::Command::new(&command_name);
            match spawn(command.args(cmdline)) {
                Ok(_status) => {}
                Err(x) => eprintln!(
                    "Failed to execute command: `{}` with error: {}",
                    &command_name, x
                ),
            };
        }
    }
    Ok(())
}
fn spawn(command: &mut process::Command) -> Result<process::ExitStatus, Box<dyn Error>> {
    Ok(command.spawn()?.wait()?)
}
