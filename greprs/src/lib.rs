use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub _query: String,
    pub _filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let _query = args[1].clone();
        let _filename = args[2].clone();

        Ok(Self { _query, _filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config._filename).expect("file not found!");

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);

    Ok(())
}
