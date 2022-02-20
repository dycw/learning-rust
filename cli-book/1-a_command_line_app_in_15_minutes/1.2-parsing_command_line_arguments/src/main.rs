// Getting the arguments

fn getting_the_arguments() {
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");
}

// CLI arguments as data type

struct Cli1 {
    pattern: String,
    path: std::path::PathBuf,
}

fn cli_arguments_as_data_type() {
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");
    let args = Cli1 {
        pattern,
        path: std::path::PathBuf::from(path),
    };
}

// Parsing CLI arguments with Clap

use clap::Parser;

#[derive(Parser)]
struct Cli2 {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn parsing_cli_arguments_with_clap() {
    let args: Cli2 = Cli2::parse();

    println!(
        "You parsed pattern = {:?}, path = {:?}",
        args.pattern, args.path
    );
}

fn main() {
    parsing_cli_arguments_with_clap();
}
