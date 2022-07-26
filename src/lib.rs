use std::{env, fs};
use std::env::args;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let result = if &config.ignore_case {
        search(&config.query, &contents)
    } else {
        search_case_sensitive(&config.query, &contents)
    };
    for line in result {
        println!("{}", line)
    }
    Ok(())
}

fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(query))
        .collect()
}

pub struct Config {
    query: String,
    filename: String,
    ignore_case: bool,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item=String>) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(query) => query,
            None => return Err("Didn't get a query string")
        };
        let filename = match args.next() {
            Some(filename) => filename,
            None => return Err("Didn't get file name")
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config { query, filename, ignore_case })
    }
}

#[cfg(test)]
mod tests {
    use crate::{search, search_case_sensitive};

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn case_insensitive() {
        let query = "RUST";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_sensitive(query, contents))
    }
}