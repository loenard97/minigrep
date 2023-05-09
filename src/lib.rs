use std::fs;
use std::io::{self, Read};
use std::error::Error;
use clap::Parser;

mod print;
mod search;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    query: Option<String>,
    file_name: Option<String>,
    #[arg(short, long, default_value_t = true)]
    case_sensitive: bool,
    #[arg(short, long, default_value_t = false)]
    pretty_print: bool,
}

#[derive(Clone)]
pub struct Config {
    pub query: Option<String>,
    pub file_name: Option<String>,
    pub case_sensitive: bool,
    pub pretty_print: bool,
    pub stdin_as_input: bool,
}

impl Config {
    pub fn new() -> Self {
        let args = Args::parse();

        let query = args.query.clone();
        let file_name = args.file_name.clone();
        let case_sensitive = args.case_sensitive;
        let pretty_print = args.pretty_print;
        let stdin_as_input = false;

        Config { query, file_name, case_sensitive, pretty_print, stdin_as_input }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_name.unwrap())?;
    let mut results: Vec<&str> = Vec::new();
    let mut buffer = String::new();

    // query
    if config.stdin_as_input {
        let stdin = io::stdin();
        stdin.lock().read_to_string(&mut buffer).expect("Could not read stdin.");
        results.push(&buffer);
    } else {
        results = if config.case_sensitive {
            search::case_sensitive(&config.query.unwrap(), &contents)
        } else {
            search::case_insensitive(&config.query.unwrap(), &contents)
        };
    }

    // print
    if config.pretty_print {
        print::pretty_print(results);
    } else {
        print::print(results);
    }

    Ok(())
}
