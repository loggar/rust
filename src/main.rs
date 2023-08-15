extern crate lib_root;

use lib_root::sum_even_numbers_in_range;

fn main() {
    let start = 1;
    let end = 10;

    let sum = sum_even_numbers_in_range(start, end);
    println!("The sum of even numbers between {} and {} is: {}", start, end, sum);
}
