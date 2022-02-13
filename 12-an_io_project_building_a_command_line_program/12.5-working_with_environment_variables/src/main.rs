use std::{env, process};

use working_with_environment_variables::{run_2, Config2};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config2::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run_2(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
