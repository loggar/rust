fn immutable_variables() -> i32 {
    let x = 5;
    // x = 6;
    println!("The value of x is: {}", x);
    return x;
}

#[test]
fn immut_test() {
    let x = immutable_variables();

    // assert_eq!( 6,x, "{} is assigned to x", 6);
    assert_eq!(5, x, "{} is assigned to x", 5);
}
