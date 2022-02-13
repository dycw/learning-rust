// Writing a Failing Test
use std::{env, process};

use developing_the_librarys_functionality_with_test_driven_development::{run, Config};

fn writing_a_failing_test() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new_1(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

fn main() {
    writing_a_failing_test();
}
