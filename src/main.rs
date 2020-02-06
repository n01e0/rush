mod rush;

extern crate signal_simple;

use std::io::{self, Read};

fn main() {
    sig_init();
    loop {
        let mut shell = rush::Status::new();
        shell.prompt("💩").flush();
        let mut line = readln();
        shell.exec(tokenize(&mut line)).finish();
    }
}

fn readln() -> String {
    let mut stdin = io::stdin();
    let mut line: Vec<u8> = Vec::new();
    let mut buf = [0; 1];
    loop {
        stdin.read(&mut buf).unwrap_or(0);
        match buf[0] {
            0 => break,
            0xa => {
                line.append(&mut buf.to_vec());
                break;
            },
            _ => line.append(&mut buf.to_vec()),
        }
    }

    String::from_utf8(line).unwrap_or("exit".to_string())
}

fn tokenize<'a>(line: &'a mut String) -> Vec<&'a str> {
    if line.len() == 0 {
        return vec!("exit");
    }
    line.retain(|r| r != '\n');
    line.split(' ').collect::<Vec<&str>>()
}

fn sig_init() {
    use signal_simple::signal::*;
    ignore(SIGINT);
    ignore(SIGTERM);
}
