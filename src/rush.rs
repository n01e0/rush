extern crate colored;
extern crate dirs;

use colored::*;
use std::env;
use std::path::Path;
use std::io::{Write, stdout};
use std::process::{ Command, exit };

pub struct Status {
    cwd: String,
    code: i32,
    prompt: String
}

impl Status {
    pub fn new() -> Status {
        Status {
            cwd: String::new(),
            code: 0,
            prompt: String::new(),
        }
    }

    pub fn flush(&mut self) -> &mut Status {
        self.cwd = env::current_dir().unwrap().to_str().unwrap().to_string();
        println!("{}", self.cwd);
        print!("{}", self.prompt);
        stdout().flush().unwrap();
        self
    }

    pub fn prompt(&mut self, content: &str) -> &mut Status {
        self.prompt = content.to_string();
        self
    }

    pub fn exec(&mut self, token: Vec<&str>) -> &mut Status {
        let eof = std::str::from_utf8(&[0]).unwrap();
        let command = token[0];
        let args: Option<Vec<&str>> = if token.len() > 1 {
            Some(token[1..].to_vec())
        } else {
            None
        };
        match command  {
            c if c == eof        => exit(0),
            "exit"               => exit(0),
            "cd"                 => self.chdir(args),
            "getenv"             => self.getenv(args),
            "setenv"             => self.setenv(args),
            bin                  => self.launch(bin, args),
        };
        self
    }

    pub fn finish(&mut self) -> &mut Status {
        self.cwd = dirs::home_dir().unwrap().to_str().unwrap().to_string();
        self
    }

    fn setenv(&mut self, args: Option<Vec<&str>>) {
        match args {
            Some(command) => {
                match command.len() {
                    1 => {
                        let command: Vec<&str> = command[0].split('=').collect();
                        if command.len() == 2 {
                            env::set_var(command[0], command[1]);
                        } else {
                            eprintln!("{}", format!("Usage: setenv <KEY=VALUE|KEY VALUE>").red());
                        }
                    },
                    2 => env::set_var(command[0], command[1]),
                    _ => eprintln!("{}", format!("Usage: setenv <KEY=VALUE|KEY VALUE>").red()),
                }
            },
            None => eprintln!("{}", format!("Usage: setenv <KEY=VALUE|KEY VALUE>").red()), 
        }

        self.code = 0;
    }

    fn getenv(&mut self, args: Option<Vec<&str>>) {
        match args {
            Some(keys) => {
                for key in keys {
                    match env::var(key) {
                        Ok(val) => println!("{}: {:?}", key, val),
                        Err(err) => eprintln!("{}", format!("could not interpret {}: {}", key, err).red()),
                    }
                }
            },
            None => eprintln!("{}", "Usage: getenv <env_key(s)>".red()),
        }
        self.code = 0;
    }

    fn chdir(&mut self, args: Option<Vec<&str>>) {
        match args {
            Some(path) => {
                let path = Path::new(path[0]);
                if let Err(err) = env::set_current_dir(path) {
                    eprintln!("{}", format!("{}", err).red());
                    eprintln!("{}", "Usage: cd <dir>".red());
                    self.code = 1;
                }
                self.code = 0;
            },
            None => {
                match dirs::home_dir() {
                    Some(home) => if let Err(err) = env::set_current_dir(home.as_path()) {
                        eprintln!("{}", format!("{}", err).red());
                        self.code = 1;
                    },
                    None => eprintln!("{}", "Usage: cd <dir>".red()),
                }
                self.code = 0;
            }
        }
    }

    fn launch(&mut self, command: &str, args: Option<Vec<&str>>) {
        let mut proc = Command::new(command);
        if let Some(args) = args {
            proc.args(&args);
        }
        self.code = match proc.status() {
            Ok(command) => {
                command.code().unwrap()
            },
            Err(err) => {
                eprintln!("{}", format!("Command not found: {}", err).red());
                1
            }
        };
    }

}

