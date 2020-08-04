use std::{fs, error::Error, env};

mod config;
pub use config::Config;

pub fn search<'a>(query: &str, content: &'a mut str, sensitive: bool) -> Vec<&'a str> {
    //query is orginated in an instance of the Config type, and as such immutable
    let mut query = query.to_string(); //an mutable copy of it
    if !sensitive {
        query = query.to_lowercase();
        content.make_ascii_lowercase();
    }
    let mut v: Vec<&str> = Vec::new();
    for line in content.lines() {
        if line.contains(&query) {
            v.push(line);
        }
    }
    v
}

pub fn run(con: &Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(con.get_file()); 
    match content {
        Result::Ok(mut s) => {
            let v = search(con.get_query(), &mut s, env::var("CASE_INSENSITIVE").is_err());
            for line in v {
                println!("{}", line);
            }
            Ok(())
        },
        Result::Err(e) => Err(Box::new(e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn search_works() {
        if env::var("CASE_INSENSITIVE").is_err() {
            let mut content = String::from("example\n SOMthing som \n for text \n\n\n something\n not intersting");
            assert_eq!(vec![" something"], search("something", &mut content, true));
        } else {
            let mut content = String::from("tesTiNG INSensitivitY???\n\n\n hopefully 887\n kyTRR");
            assert_eq!(vec!["tesTiNG INSensitivitY???"], search("insE", &mut content, false));
        }
    }
}
