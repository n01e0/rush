extern crate colored;
extern crate dirs;
extern crate hostname;

use colored::*;
use std::env;
use std::path::{Path, PathBuf};
use std::io::{Write, stdout};
use std::process::{ Command, exit };

pub struct Shell {
    user: String,
    cwd: PathBuf,
    branch: String,
    code: i32,
    prompt: String
}

impl Shell {
    pub fn new() -> Shell {
        Shell {
            user: String::new(),
            cwd: PathBuf::new(),
            branch: String::new(),
            code: 0,
            prompt: String::new(),
        }
    }

    pub fn flush(&mut self) -> &mut Shell {
        self.user = if let Ok(output) = 
            Command::new("whoami").output() {
                let mut name = output.stdout;
                name.retain(|c| *c != 0xa);
                format!("{}/{}>", hostname::get().unwrap().into_string().unwrap(), String::from_utf8(name).unwrap())
        }else{"".to_string()};
        self.cwd = env::current_dir().unwrap();
        self.branch = if let Ok(output) = 
            Command::new("git").args(&["symbolic-ref", "--short", "HEAD"]).output() {
                let mut currnt_branch = output.stdout;
                currnt_branch.retain(|c| *c != 0xa);
                format!("[{}]", 
                        String::from_utf8(currnt_branch).unwrap_or("".to_string())
                )
        }else{"".to_string()};
        let user = format!("{}", self.user).on_green();
        let cwd = format!("{}", self.cwd.to_str().unwrap()).red().on_cyan();
        let branch = format!("{}", self.branch).on_red();
        println!("{}{}{}", user, cwd, branch);
        print!("{}", self.prompt);
        stdout().flush().unwrap();
        self
    }

    pub fn prompt(&mut self, content: &str) -> &mut Shell {
        self.prompt = content.to_string();
        self
    }

    pub fn exec(&mut self, token: Vec<&str>) -> &mut Shell {
        let eof = std::str::from_utf8(&[0]).unwrap();
        let command = if token.len() > 0 {
            token[0]
        } else {
            "clear"
        };
        let args: Option<Vec<&str>> = if token.len() > 1 {
            Some(token[1..].to_vec())
        } else {
            None
        };
        self.code = match command  {
            c if c == eof        => exit(0),
            "exit"               => exit(0),
            "cd"                 => crate::rush::chdir(args),
            "getenv"             => crate::rush::getenv(args),
            "setenv"             => crate::rush::setenv(args),
            bin                  => crate::rush::launch(bin, args),
        };
        self
    }

    pub fn finish(&mut self) -> &mut Shell {
        self.cwd = dirs::home_dir().unwrap();
        self
    }
}

fn setenv(args: Option<Vec<&str>>) -> i32 {
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

    return 0;
}

fn getenv(args: Option<Vec<&str>>) -> i32 {
    match args {
        Some(keys) => {
            for key in keys {
                match env::var(key) {
                    Ok(val) => {
                        println!("{}: {:?}", key, val);
                        return 0;
                    },
                    Err(err) => {
                        eprintln!("{}", format!("could not interpret {}: {}", key, err).red());
                        return 1;
                    },
                }
            }
        },
        None => {
            eprintln!("{}", "Usage: getenv <env_key(s)>".red());
            return 1;
        },
    }
    return 0;
}

fn chdir(args: Option<Vec<&str>>) -> i32 {
    match args {
        Some(path) => {
            let path = Path::new(path[0]);
            if let Err(err) = env::set_current_dir(path) {
                eprintln!("{}", format!("{}", err).red());
                eprintln!("{}", "Usage: cd <dir>".red());
                return 1;
            }
            return 0;
        },
        None => {
            match dirs::home_dir() {
                Some(home) => if let Err(err) = env::set_current_dir(home.as_path()) {
                    eprintln!("{}", format!("{}", err).red());
                    return 1;
                },
                None => eprintln!("{}", "Usage: cd <dir>".red()),
            }
            return 0;
        }
    }
}

//fn alias(args: Option<Vec<&str>>) -> i32 {
//    match args {
//        Some(command) => {
//            match command.len() {
//                1 => {
//                    let command: Vec<&str> = command[0].split('=').collect();
//                    if command.len() == 2 {
//                        env::set_var(command[0], command[1]);
//                    } else {
//                        eprintln!("{}", format!("Usage: setenv <KEY=VALUE|KEY VALUE>").red());
//                    }
//                },
//                2 => env::set_var(command[0], command[1]),
//                _ => eprintln!("{}", format!("Usage: setenv <KEY=VALUE|KEY VALUE>").red()),
//            }
//        },
//        None => eprintln!("{}", format!("Usage: setenv <KEY=VALUE|KEY VALUE>").red()), 
//    }
//
//    return 0;
//}

fn launch(command: &str, args: Option<Vec<&str>>) -> i32 {
    let mut proc = Command::new(command);
    if let Some(args) = args {
        proc.args(&args);
    }
    match proc.status() {
        Ok(command) => {
            command.code().unwrap()
        },
        Err(err) => {
            eprintln!("{}", format!("Command not found: {}", err).red());
            1
        }
    }
}
