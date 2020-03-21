use std::env;
use std::process;

use grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error when parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(error) = grep::run(config) {
        println!("{}", error);
        process::exit(1);
    }
}
