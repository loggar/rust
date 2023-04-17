fn mutable_variables() -> i32 {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    return x;
}

#[test]
fn immut_test() {
    let x = mutable_variables();

    assert_eq!( 6,x, "{} is assigned to x", 6);

    // x = 7;
    // cannot mutate immutable variable `x`rust-analyzerneed-mut
}
