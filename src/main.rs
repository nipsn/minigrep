use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    /*
        Cool note: you can use args.iter() to unlock map and flatMap!
     */

    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let content = fs::read_to_string(config.file_path)
        .expect("What you doin brother. No file there");

    println!("File text:\n{content}")
}

struct Config {
    query: String,
    file_path: String
}

fn parse_config(args: &[String]) -> Config {
    Config { query: args[1].clone(), file_path: args[2].clone() }
}
