// Writing a Failing Test
use std::{env, process};

use developing_the_librarys_functionality_with_test_driven_development::{run_1, Config1};

fn splitting_code_into_a_library_crate() {
    let args: Vec<String> = env::args().collect();

    let config = Config1::new_1(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run_1(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

fn main() {
    splitting_code_into_a_library_crate();
}
