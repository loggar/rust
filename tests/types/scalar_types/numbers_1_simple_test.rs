use lib_root::lib_types::get_type_of;

#[test]
fn number_int_test() {
    let i1 = 42;
    println!("i1 {} {}", i1, get_type_of(&i1));

    assert_eq!("i32", get_type_of(&i1), "get_type_of({}) should equal to {}", i1, "i32");

    let i2: i64 = 42;

    assert_eq!("i64", get_type_of(&i2), "get_type_of({}) should equal to {}", i2, "i64");
}

#[test]
fn number_float_test() {
    let f1 = 2.0;
    println!("i1 {} {}", f1, get_type_of(&f1));

    assert_eq!("f64", get_type_of(&f1), "get_type_of({}) should equal to {}", f1, "f64");

    let f2: f32 = 2.0;

    assert_eq!("f32", get_type_of(&f2), "get_type_of({}) should equal to {}", f2, "f32");
}

#[test]
fn number_operation_test() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    assert_eq!(15, sum, "sum: {} should equal to {}", get_type_of(&sum), 15);
    assert_eq!(
        91.2,
        difference,
        "difference: {} should equal to {}",
        get_type_of(&difference),
        91.2
    );
    assert_eq!(
        120,
        product,
        "product: {} should equal to {}",
        get_type_of(&product),
        120
    );
    assert_eq!(
        1.7608695652173911,
        quotient,
        "quotient: {} should equal to {}",
        get_type_of(&quotient),
        1.7608695652173911
    );
    assert_eq!(
        -1,
        truncated,
        "truncated: {} should equal to {}",
        get_type_of(&truncated),
        -1
    );
    assert_eq!(
        3,
        remainder,
        "remainder: {} should equal to {}",
        get_type_of(&remainder),
        3
    );
}
