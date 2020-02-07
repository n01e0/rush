mod rush;

extern crate signal_simple;

use std::io::{self, Read};
use std::env;

fn main() {
    sig_init();
    loop {
        let mut shell = rush::Shell::new();
        shell.prompt("💩 ").flush();
        let mut line = readln();
        shell.exec(tokenize(&mut line).iter().map(|x| x.as_str()).collect::<Vec<&str>>()).finish();
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

fn tokenize(line: &mut String) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    if line.len() == 0 {
        return vec!("exit".to_string());
    }
    line.retain(|r| r != '\n');
    for tok in line.split_whitespace().map(|x| x.to_string()).collect::<Vec<String>>().iter_mut() {
        if tok.starts_with("$") {
            tok.remove(0);
            match env::var(&tok) {
                Ok(val) => tokens.push(val),
                Err(_) => tokens.push("".to_string()),
            };
        } else {
            tokens.push(tok.to_string());
        }
    }
    tokens
}

fn sig_init() {
    use signal_simple::signal::*;
    ignore(SIGINT);
    ignore(SIGTERM);
}
