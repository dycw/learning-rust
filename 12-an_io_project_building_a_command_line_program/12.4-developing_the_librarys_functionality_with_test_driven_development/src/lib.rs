use std::error::Error;
use std::fs;

pub struct Config1 {
    pub query: String,
    pub filename: String,
}

impl Config1 {
    pub fn new_1(args: &[String]) -> Result<Config1, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config1 { query, filename })
    }
}

pub fn run_1(config: Config1) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}
