// Reading the Argument Values

use std::env;

fn reading_the_argument_values() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}

// Saving the Argument Values in Variables

fn saving_the_argument_values_in_variables() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}

fn main() {
    saving_the_argument_values_in_variables();
}
