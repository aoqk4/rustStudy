use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub _query: String,
    pub _filename: String,
    pub _ignore_case: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let _query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let _filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let _ignore_case = env::var("IGNORE_CASE").is_err();

        Ok(Config {
            _query,
            _filename,
            _ignore_case,
        })
    }

    // pub fn new(args: &[String]) -> Result<Self, &'static str> {
    //     Ok(Self { _query, _filename })
    // }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // let f = File::open(config._filename).expect("file not found!");

    let contents = fs::read_to_string(config._filename)?;

    let results = if config._ignore_case {
        search_case_insensitive(&config._query, &contents)
    } else {
        search(&config._query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
