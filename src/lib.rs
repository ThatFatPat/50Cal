extern crate directories;
extern crate rustyline;
use directories::BaseDirs;
use rustyline::{error::ReadlineError, Editor};
use std::{
    env,
    error::Error,
    io::{self, Write},
    process,
};

pub struct Context {
    last_status: Option<process::ExitStatus>,
    editor: Editor<()>,
}

impl Context {
    pub fn new() -> Self {
        let mut e = Editor::<()>::new();
        if let Some(base_dirs) = BaseDirs::new() {
            e.load_history(&base_dirs.home_dir().join(".50cal_history"))
                .unwrap_or_default()
        }
        Self {
            last_status: None,
            editor: e,
        }
    }
}
impl Default for Context {
    fn default() -> Self {
        Context::new()
    }
}

pub fn process_line(ctx: &mut Context) -> Result<(), Box<dyn Error>> {
    let readline = ctx.editor.readline(&prompt(&ctx)?);
    match readline {
        Ok(line) => {
            ctx.editor.add_history_entry(line.as_str());
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
        Err(ReadlineError::Interrupted) => {
            process::exit(0);
        }
        Err(ReadlineError::Eof) => Ok(()),
        Err(err) => {
            println!("Error: {:?}", err);
            Ok(())
        }
    }
}
fn spawn(command: &mut process::Command) -> Result<process::ExitStatus, Box<dyn Error>> {
    Ok(command.spawn()?.wait()?)
}

pub fn prompt(ctx: &Context) -> Result<String, Box<dyn Error>> {
    let mut ret = String::new();
    if let Some(code) = ctx
        .last_status
        .filter(|status| !status.success())
        .and_then(|status| status.code())
    {
        ret.push_str(&format!("\nReturn Code: {}", code));
    }
    ret.push_str(&format!("{} @ ", env::current_dir()?.display()));
    Ok(ret)
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
