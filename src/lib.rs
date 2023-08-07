use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, Box<dyn Error>> {
        if args.len() < 3 {
            return Err("not enough arguments".into());
        }
        let query = args[2].clone();
        let filename = args[1].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.filename)?;
    let found = search(&config.query, &content);

    if found.is_empty() {
        println!("No results found");
    } else {
        for elem in &found {
            println!("{}", elem);
        }
    }

    Ok(())
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}