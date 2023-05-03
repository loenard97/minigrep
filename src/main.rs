use std::{env, fs};

struct Config {
    query: String,
    file_name: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    let contents = fs::read_to_string(config.file_name);
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_name = args[2].clone();
    Config { query, file_name }
}
