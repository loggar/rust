#[test]
fn shadow_test() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("The value of x is: {}", x);

    let expected = (5 + 1) * 2;
    assert_eq!(x, expected, "x should be now {}", expected);
}

#[test]
fn shadow_type_mismatch_test() {
    let spaces = "   ";

    // spaces  spaces.len();
    // error[E0308]: mismatched types

    // assert_eq!(spaces, 3, "spaces should be now {}", 3);
    assert_eq!(spaces.len(), 3, "spaces should be now {}", 3);
}
