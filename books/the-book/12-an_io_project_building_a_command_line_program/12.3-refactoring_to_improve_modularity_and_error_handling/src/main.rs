// Extracting the Argument Parser

use std::env;
use std::fs;

fn extracting_the_argument_parser() {
    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_config_1(&args);

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

fn parse_config_1(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}

// Grouping Configuration Values

fn grouping_configuration_values() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config_2(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

struct Config1 {
    query: String,
    filename: String,
}

fn parse_config_2(args: &[String]) -> Config1 {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config1 { query, filename }
}

// Creating a Constructor for Config

fn creating_a_constructor_for_config() {
    let args: Vec<String> = env::args().collect();

    let config = Config1::new_1(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

impl Config1 {
    fn new_1(args: &[String]) -> Config1 {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config1 { query, filename }
    }
}

// Improving the Error Message

fn improving_the_error_message() {
    let args: Vec<String> = env::args().collect();

    let config = Config1::new_2(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

impl Config1 {
    fn new_2(args: &[String]) -> Config1 {
        if args.len() < 3 {
            panic!("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Config1 { query, filename }
    }
}

// Returning a Result from new Instead of Calling panic!

use std::process;

fn returning_a_result_from_new_instead_of_calling_panic() {
    let args: Vec<String> = env::args().collect();

    let config = Config1::new_3(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

impl Config1 {
    fn new_3(args: &[String]) -> Result<Config1, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config1 { query, filename })
    }
}

// Extracting Logic from main

fn extracting_logic_from_main() {
    let args: Vec<String> = env::args().collect();

    let config = Config1::new_3(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    run_1(config);
}

fn run_1(config: Config1) {
    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

// Returning Errors from the run Function

use std::error::Error;

fn returning_errors_from_the_run_function() {
    let args: Vec<String> = env::args().collect();

    let config = Config1::new_3(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    run_2(config);
}

fn run_2(config: Config1) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}

// Handling Errors Returned from run in main

fn handling_errors_returned_from_run_in_main() {
    let args: Vec<String> = env::args().collect();

    let config = Config1::new_3(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run_2(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

// Splitting Code into a Library Crate

use refactoring_to_improve_modularity_and_error_handling::{run_3, Config2};

fn splitting_code_into_a_library_crate() {
    let args: Vec<String> = env::args().collect();

    let config = Config2::new_4(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run_3(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

fn main() {
    splitting_code_into_a_library_crate();
}
