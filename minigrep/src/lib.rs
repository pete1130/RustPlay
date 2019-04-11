use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let read = fs::read_to_string(config.filename)?;
    println!("The contents of the file are:\n{}", read);

    Ok(())
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            panic!("Not enough arguments!")
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
