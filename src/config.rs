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
    pub fn new(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enought arguments");
        }
        Ok(Config {
            //In order to independent of "args" lifetime
            query: args[2].clone(),
            file: args[1].clone(),
        })
    }
}


