use std::env;

fn main() {
    // Read cli args
    let args: Vec<String> = env::args().collect();
    println!("Program Args: {:?}", args);

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);
}
