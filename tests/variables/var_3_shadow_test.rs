#[test]
fn shadow_test() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("The value of x is: {}", x);

    let expected = (5 + 1) * 2;
    assert_eq!(expected, x, "x should be now {}", expected);
}

#[test]
fn shadow_type_mismatch_test() {
    let spaces: &str = "   ";

    // spaces  spaces.len();
    // error[E0308]: mismatched types

    // assert_eq!( 3,spaces, "spaces should be now {}", 3);
    assert_eq!(3, spaces.len(), "spaces should be now {}", 3);
}
