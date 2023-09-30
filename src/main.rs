use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    /*
        Cool note: you can use args.iter() to unlock map and flatMap!
     */

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let content = fs::read_to_string(file_path)
        .expect("What you doin brother. No file there");

    println!("File text:\n{content}")
}
