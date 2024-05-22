#![allow(dead_code)]

use std::env::{var,Args, args};
use std::fs;
use std::error::Error;
use std::process::exit;

struct Config {
    query: String,
    filename: String,
    is_case_sensitive: bool,
}

impl Config {
    fn new(mut args:Args) -> Result<Config, String> {
        args.next(); // Skip the first argument (program name)
        
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string".to_string()),
        };
        
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name".to_string()),
        };

        let is_case_sensitive = var("CASE_INSENSITIVE").is_err(); // if the env var is set, don't return an error
	/*
	This means if the env var is set (whatever value) case insensitive search will be performed
	*/

        return Ok(Config {query, filename, is_case_sensitive} );
    }

    fn search(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let contents = fs::read_to_string(&self.filename)?;

        let results = if self.is_case_sensitive { search_case_sensitive(&self.query, &contents) }
                      else { search_case_insensitive(&self.query, &contents) };
	
	println!("{:?}",results);

	// `Result` is a type that represents either success ([`Ok`]) or failure ([`Err`]).
        return Ok(results);
    }
}

fn search_case_sensitive(query: &str, contents:&str) -> Vec<String> {
    contents.lines()
        .filter(|line| line.contains(&query))
        .map(|line| line.to_string())
        .collect()
}

fn search_case_insensitive(query: &str, contents: &str) -> Vec<String> {
    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .map(|line| line.to_string())
        .collect()
}

fn main() {
    let config = Config::new(args()).unwrap_or_else(|err| { eprintln!("Problem parsing arguments: {}", err);  exit(1);});

    match config.search() {
        Ok(results) => {
            for line in results {
                println!("{}", line);
            }
        },
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            exit(1);
        },
    }
}
