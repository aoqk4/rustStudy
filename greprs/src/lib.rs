use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use test::search;

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

    for line in search(&config._query, &config._contents) {
        println!("{line}");
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use std::result;

    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "/
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let mut results = Vec::new();

        for line in contents.lines() {
            if line.contains(query) {
                results.push(line);
            }
        }
        results
    }

    #[cfg(test)]
    mod tests {
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
}
