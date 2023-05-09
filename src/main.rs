use std::process;

use minigrep::Config;

fn main() {
    let config = Config::new();

    if let Err(err) = minigrep::run(config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}
