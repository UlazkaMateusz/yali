use anyhow::Result;
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
    let tokens = scanner::scan_tokens(&source)?;

    for token in tokens {
        println!("{:#?}", token);
    }

    Ok(())
}
