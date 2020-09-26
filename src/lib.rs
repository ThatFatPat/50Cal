use std::{
    env,
    error::Error,
    io::{self, Write},
    process,
};
#[derive(Default)]
pub struct Context {
    last_status: Option<process::ExitStatus>,
}

impl Context {
    pub fn new() -> Self {
        Self::default()
    }
}

pub fn process_cmdline(line: &str, ctx: &mut Context) -> Result<(), Box<dyn Error>> {
    let mut cmdline = line.split_whitespace();
    let command_name = match cmdline.next() {
        Some(x) => x,
        None => return Ok(()),
    };
    match command_name {
        "exit" => process::exit(0),
        "cd" => chdir(cmdline),
        _ => {
            let mut command = process::Command::new(&command_name);
            match spawn(command.args(cmdline)) {
                Ok(status) => ctx.last_status = Some(status),
                Err(x) => eprintln!(
                    "Failed to execute command: `{}` with error: {}",
                    &command_name, x
                ),
            };
            Ok(())
        }
    }
}
fn spawn(command: &mut process::Command) -> Result<process::ExitStatus, Box<dyn Error>> {
    Ok(command.spawn()?.wait()?)
}
pub fn prompt(ctx: &Context) -> Result<(), Box<dyn Error>> {
    if let Some(code) = ctx
        .last_status
        .filter(|status| !status.success())
        .and_then(|status| status.code())
    {
        println!("\nReturn Code: {}", code)
    }
    print!("{} $ ", env::current_dir()?.display());
    io::stdout().flush()?;
    Ok(())
}
pub fn chdir<'a>(mut args: impl Iterator<Item = &'a str>) -> Result<(), Box<dyn Error>> {
    if let Some(dir) = args.next() {
        match args.next() {
            Some(_x) => panic!("Too many arguments to cd"),
            None => Ok(env::set_current_dir(dir)?),
        }
    } else {
        panic!("Not enough arguments for cd")
    }
}
