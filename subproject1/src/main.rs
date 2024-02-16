use subproject2;

fn main() {
    let num: usize = 10;
    println!(
        "Hello, world! subproject2::add({num}, {num}) is {}!",
        subproject2::add(num, num)
    );
}
