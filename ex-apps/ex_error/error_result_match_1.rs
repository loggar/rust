use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let file = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        }
    };

    let metadata = file.metadata().expect("Unable to retrieve metadata");
    println!("Is it a file? {}", metadata.is_file());
}

// rustc ./ex-apps/ex_error_simple/error_result_match_1.rs --out-dir ./target.ex-apps/playpen
// target.ex-apps/playpen/error_result_match_1
