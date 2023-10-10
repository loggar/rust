extern crate greprs; // get lib using the package name

use std::env;
use std::process;

use greprs::Config;

// cargo run -- to in/poem.txt
// IGNORE_CASE=0 cargo run -- to in/poem.txt

// cargo run -- to > out/output.txt
// cargo run -- to in/poem.txt out/output.txt
// IGNORE_CASE=0 cargo run -- to in/poem.txt > out/output.txt
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    println!("Ignore case {}", config.ignore_case);

    if let Err(e) = greprs::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
