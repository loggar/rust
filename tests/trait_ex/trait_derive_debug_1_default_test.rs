#[derive(Debug)]
#[allow(dead_code)]
struct Point {
    x: i32,
    y: i32,
}

// The #[derive(Debug)] line in your Rust code is an attribute that automatically derives the Debug trait for the type
// it's applied to.

// In Rust, traits are a way to define shared behaviour across types. The Debug trait is used for formatting an instance
// of a type for output, typically for debugging purposes. By deriving the Debug trait for a type, you're enabling the
// ability to print instances of that type using the {:?} or {:#?} format specifiers in your print statements.

#[test]
fn derive_debug_default_trait_test() {
    let origin = Point { x: 0, y: 0 };

    // The {:?} format specifier is used to print an instance of a type using the Debug trait.
    // println!("The origin is: {:?}", origin);
    // The {:#?} format specifier is used to print an instance of a type using the Debug trait with pretty-printing.
    println!("The origin is: {:#?}", origin);
    // Without the #[derive(Debug)] attribute, trying to print the struct would result in a compile error, because the
    // Debug trait is not implemented for the Point type.

    assert!(format!("{:?}", origin) == "Point { x: 0, y: 0 }", "debug trait");
    assert!(
        format!("{:#?}", origin) == "Point {\n    x: 0,\n    y: 0,\n}",
        "debug trait"
    );
}
