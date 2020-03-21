use std::env;
use std::process;

use grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Error when parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(error) = grep::run(config) {
        eprintln!("{}", error);
        process::exit(1);
    }
}
