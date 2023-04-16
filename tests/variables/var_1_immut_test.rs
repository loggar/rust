fn immutable_variables() -> i32 {
    let x = 5;
    // x = 6;
    println!("The value of x is: {}", x);
    return x;
}

// 

#[test]
fn var_1_immut_test_1() {
    let x = immutable_variables();

    // assert_eq!(x, 6, "{} is assigned to x", 6);
    assert_eq!(x, 5, "{} is assigned to x", 5);
}
