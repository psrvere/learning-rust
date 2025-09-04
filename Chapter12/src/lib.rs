use std::{fs, env};
use std::error::Error;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}


impl Config {
    // impl Iterator implies that args can be of any type that implements the Iterator type
    // and returns string items
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // ignoring first value which is name of program

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get file path string"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config{query, file_path, ignore_case})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };

    for result in results {
        println!("{result}");
    }
    Ok(())
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
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "safe";
        let contents = "\
Rust:
safe, fast, productive.
Pick three";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents),
        )
    }

    #[test]
    fn case_insensitive() {
        let query = "RusT";
        let contents = "\
Rust:
Safe, fast, productive.
Pick three
Trust me.";
        
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents),
        )
    }
}