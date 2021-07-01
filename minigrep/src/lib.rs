use std::error::Error;
use std::fs;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        let n = args.len() - 1;
        let min = 2;
        if n >= min {
            let query = args[1].clone();
            let filename = args[2].clone();
            Ok(Config { query, filename })
        } else {
            let _msg = format!("You need at least {} arguments; you passed in {}", min, n);
            Err("what")
        }
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}
