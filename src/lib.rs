use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[2].clone();
        let filename = args[1].clone();

        Ok(Config {
            query,
            filename,
        })
    }
}

pub fn run(config: Config) {
    let content = fs::read_to_string(config.filename).expect("Something went wrong reading the file");
    let found = search(&config.query, &content);

    if found.is_empty() {
        println!("No results found");
        return;
    }

    for elem in found {
        println!("{}", elem);
    }
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}