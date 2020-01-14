use std::env;
use std::path::Path;
use std::io::{self, Write};
use std::process::{ Command, exit };

fn main() {
    let mut stdout = io::stdout();
    loop {
        write!(stdout, "$ ").unwrap();
        stdout.flush().unwrap();
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("read error");
        line.retain(|r| r != '\n');
        let line: Vec<&str> = line.split(' ').collect();
        let command = line[0];
        let args: Option<Vec<&str>> = if line.len() > 1 {
            Some(line[1..].to_vec())
        } else {
            None
        };
        match command {
            "exit"  => exit(0),
            "cd"    => chdir(args),
            bin     => launch(bin, args),
        }
    }
}

fn launch(command: &str, args: Option<Vec<&str>>) {
    let mut proc = Command::new(command);
    if let Some(args) = args {
        proc.args(&args);
    }
    match proc.output() {
        Ok(command)    => {
            write!(io::stdout(), "{}", String::from_utf8(command.stderr).unwrap()).unwrap();
            write!(io::stderr(), "{}", String::from_utf8(command.stdout).unwrap()).unwrap();
        },
        Err(err) => eprintln!("{}", err),
    }
}

fn chdir(args: Option<Vec<&str>>) {
    match args {
        Some(path) => {
            let path = Path::new(path[0]);
            if let Err(err) = env::set_current_dir(path) {
                eprintln!("{}", err);
            }
        },
        None => eprintln!("Usage: cd <dir>"),
    }
}
