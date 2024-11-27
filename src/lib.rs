use std::env;
use std::error::Error;
use std::{fs::File, io::Read};

pub struct Config {
    query: String,
    filename: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();

    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use crate::Config;

    #[test]
    fn configのnew関数で引数queryとfilenameが無い() {
        let args: Vec<String> = vec![String::from("main")];
        if let Err(error) = Config::new(&args) {
            assert_eq!(error, "not enough arguments");
        }
    }

    #[test]
    fn configのnew関数で引数filenameが無い() {
        let args: Vec<String> = vec![String::from("main"), String::from("test")];
        if let Err(error) = Config::new(&args) {
            assert_eq!(error, "not enough arguments");
        }
    }

    #[test]
    fn configのnew関数で正常に動作する() {
        let args: Vec<String> = vec![String::from("main"), String::from("test"), String::from("test.txt")];
        let config = Config::new(&args).expect("引数が不正");
        assert_eq!(config.query, args[1]);
        assert_eq!(config.filename, args[2]);
    }
}