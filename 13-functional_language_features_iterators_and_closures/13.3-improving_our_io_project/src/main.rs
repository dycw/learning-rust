use std::{env, process};

use improving_our_io_project::{run_2, Config};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run_2(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
