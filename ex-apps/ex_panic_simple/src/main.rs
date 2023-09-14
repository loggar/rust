/*
fn main() {
    panic!("crash and burn");
}

thread 'main' panicked at 'crash and burn', src/main.rs:2:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
*/

fn main() {
    let v = vec![1, 2, 3];

    v[99];
}

// cargo run
// RUST_BACKTRACE=1 cargo run
// RUST_BACKTRACE=full cargo run
