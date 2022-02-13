use std::error::Error;
use std::fs;

pub struct Config2 {
    pub query: String,
    pub filename: String,
}

impl Config2 {
    pub fn new_4(args: &[String]) -> Result<Config2, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config2 { query, filename })
    }
}

pub fn run_3(config: Config2) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}
