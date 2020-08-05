use std::{env, process};
use rgrep::Config;

fn main() { 
    //Ownership of the iterator is passed to Config::new
    let con = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error ocurred parsing the arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = rgrep::run(&con) {
        eprintln!("Error reading the file: {}", e);
        process::exit(1);
    }
}
