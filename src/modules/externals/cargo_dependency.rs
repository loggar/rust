// $ cargo add rand
// Cargo.toml
// [dependencies]
// rand = "0.8.5"

use rand::Rng;

#[allow(dead_code)]
pub fn rand_simple() -> i32 {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("secret_number is: {}", secret_number);

    return secret_number;
}
