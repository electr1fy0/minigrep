use std::env;
use std::error::Error;
use std::fs;

pub enum Mode {
    Announce,
    Find,
}

pub struct Config {
    pub mode: Mode,
    pub query: Option<String>,
    pub file_path: Option<String>,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let mode = match args[1].as_str() {
            "announce" => Mode::Announce,
            "find" => Mode::Find,
            _ => return Err("Invalid mode. Use 'announce' or 'find'"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        match mode {
            Mode::Announce => Ok(Config {
                mode,
                query: None,
                file_path: None,
                case_sensitive,
            }),

            Mode::Find => Ok(Config {
                mode,
                query: Some(args[2].clone()),
                file_path: Some(args[3].clone()),
                case_sensitive,
            }),
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config.mode {
        Mode::Announce => {
            println!("Meow: Preparation for future features.")
        }
        Mode::Find => {
            println!("Searching for: {}", config.query.as_ref().unwrap());
            println!("In file: {}", config.file_path.as_ref().unwrap());

            let contents = fs::read_to_string(config.file_path.unwrap())?;

            let results = if config.case_sensitive {
                search(&config.query.unwrap(), &contents)
            } else {
                search_case_insensitive(&config.query.unwrap(), &contents)
            };

            for line in results {
                println!("{line}");
            }
        }
    };

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
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.
";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
