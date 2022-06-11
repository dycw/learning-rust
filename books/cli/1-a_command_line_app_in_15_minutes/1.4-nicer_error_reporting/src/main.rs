// Unwrapping

use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn unwrapping_1() {
    let args: Cli = Cli::parse();

    let result = std::fs::read_to_string(&args.path);
    let content = match result {
        Ok(content) => content,
        Err(error) => {
            panic!("Can't deal with {}, just exit here", error);
        }
    };

    println!("File content: {}", content);

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}

fn unwrapping_2() {
    let args: Cli = Cli::parse();

    let content = std::fs::read_to_string(&args.path).unwrap();

    println!("File content: {}", content);

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}

// No need to panic

fn no_need_to_panic() -> Result<(), Box<dyn std::error::Error>> {
    let args: Cli = Cli::parse();

    let result = std::fs::read_to_string(&args.path);
    let content = match result {
        Ok(content) => content,
        Err(error) => return Err(error.into()),
    };

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}

// Question Mark

fn question_mark() -> Result<(), Box<dyn std::error::Error>> {
    let args: Cli = Cli::parse();

    let content = std::fs::read_to_string(&args.path)?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}

// Providing Context

#[derive(Debug)]
struct CustomError(String);

fn providing_context_1() -> Result<(), CustomError> {
    let args: Cli = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .map_err(|err| CustomError(format!("Error reading `{:?}`: {}", &args.path, err)))?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}

use anyhow::{Context, Result as AnyhowResult};

fn providing_context_2() -> AnyhowResult<()> {
    let args: Cli = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{:?}`", &args.path))?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}

fn main() {
    providing_context_2();
}
