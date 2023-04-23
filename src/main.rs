use anyhow::Result;
use scanner::Scanner;
use std::{
    env,
    io::{self, BufRead},
    process::exit,
};

mod scanner;

fn main() -> Result<()> {
    // Get args
    let args = env::args().collect::<Vec<String>>();

    if args.len() > 2 {
        println!("Usage: yali [script]");
        exit(64);
    } else if args.len() == 2 {
        run_file(&args[1])?;
    } else {
        run_prompt()?;
    }

    Ok(())
}

fn run_prompt() -> Result<()> {
    for line in io::stdin().lock().lines() {
        run(line?)?;
    }
    Ok(())
}

fn run_file(filename: &str) -> Result<()> {
    let data = std::fs::read_to_string(filename)?;
    run(data)?;
    Ok(())
}

fn run(source: String) -> Result<()> {
    let mut scanner = Scanner::new(&source);
    let tokens = scanner.scan_tokens()?;

    for token in tokens {
        println!("{:#?}", token);
    }

    Ok(())
}

pub fn error(line: i32, message: &str) {
    report(line, "", message);
}

fn report(line: i32, where_: &str, message: &str) {
    eprintln!("[line {line}] Error {where_}: {message}");
}
