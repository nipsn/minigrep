// use core::panicking::panic;
use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    /*
        Cool note: you can use args.iter() to unlock map and flatMap!
     */

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1)
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    };

}

fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(config.file_path)?;

    println!("File text:\n{content}");

    Ok(())
}

struct Config {
    query: String,
    file_path: String
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        return Ok(Config { query: args[1].clone(), file_path: args[2].clone() } );
    }    
}
