use std::env;
use minigrep::Config;

fn main() {
    let argrs: Vec<String> = env::args().collect();
    
    let config = Config::new(&argrs);
    let config = match config {
        Ok(config) => config,
        Err(e) => {
            println!("Problem parsing arguments: {}", e);
            return;
        }
        };
        
    println!("filename: {}", config.filename);
    println!("query: {}", config.query);

    minigrep::run(config);
}
