use std::env;
use std::error::Error;
use std::{fs, vec};
pub struct Config {
    pub query: String,
    pub path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            path,
            ignore_case,
        })
    }
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut searches = Vec::new();

    content.lines().for_each(|line| {
        if line.contains(query) {
            searches.push(line.trim())
        }
    });
    searches
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut searches = Vec::new();
    let query = query.to_lowercase();
    content.lines().for_each(|line| {
        if line.to_lowercase().contains(&query) {
            searches.push(line.trim())
        }
    });
    searches
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.path)?;

    if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
            .iter()
            .for_each(|line| println!("{line:?}"));
    } else if config.ignore_case {
        search(&config.query, &contents)
            .iter()
            .for_each(|line| println!("{line:?}"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
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
        )
    }
}
