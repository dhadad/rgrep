pub struct Config {
    query: String, //A search query
    file: String, //An address whithin the user's system
}

impl Config {
    pub fn get_query(&self) -> &str {
        &(self.query.as_str())
    }
    pub fn get_file(&self) -> &str {
        &(self.file.as_str())
    }
    //The function is given an std::env::Args iterator (with ownership over it)
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();
        let file = match args.next() {
            Some(s) => s,
            None => return Err("Problematic file address, try again."),
        };
        let query = match args.next() {
            Some(s) => s,
            None => return Err("Problematic search query, try again."),
        };
        Ok(Config {
            query,
            file,
        })
    }
}


