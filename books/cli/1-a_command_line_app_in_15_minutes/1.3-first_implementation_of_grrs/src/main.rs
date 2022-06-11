// First implementation of grrs

use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn first_implementation_of_grrs() {
    let args: Cli = Cli::parse();

    println!(
        "You parsed pattern = {:?}, path = {:?}",
        args.pattern, args.path
    );

    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}

// Exercise for the reader

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

fn exercise_for_the_reader() {
    let args: Cli = Cli::parse();

    println!(
        "You parsed pattern = {:?}, path = {:?}",
        &args.pattern, &args.path
    );

    let f = File::open(&args.path).expect("could not read file");
    let mut f = BufReader::new(f);

    for line in f.by_ref().lines() {
        if line.as_ref().unwrap().contains(&args.pattern) {
            println!("{}", line.as_ref().unwrap());
        }
    }
}

fn main() {
    exercise_for_the_reader();
}
