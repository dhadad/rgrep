use std::{env, process};
use rgrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let con = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Error passing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = rgrep::run(&con) {
        eprintln!("Error reading the file: {}", e);
        process::exit(1);
    }
}
