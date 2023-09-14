use std::fs::File;

fn main() {
    // if OK unwrap will return a value inside OK.
    // if Error unwrap will call panic! macro.
    let f = File::open("hello.txt").unwrap();

    // if OK unwrap will return a value inside OK.
    // if Error unwrap will call panic! macro with the given message.
    let g = File::open("hello.txt").expect("Failed to open hello.txt");
}
