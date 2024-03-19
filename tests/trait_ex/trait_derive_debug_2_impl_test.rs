// The Debug trait in Rust, when used with the :#? format specifier for "pretty printing", does typically use a new line
// and four spaces for indentation. This is the default behavior provided by the standard library's implementation of
// Debug for structs and enums.

// However, it's important to note that this behavior can be overridden. If you implement the Debug trait manually for
// your type, you can control exactly how it's formatted. This includes the number of spaces used for indentation,
// whether or not to use new lines, and any other formatting details.

use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point {{\n\tx: {},\n\ty: {},\n}}", self.x, self.y)
    }
}

#[test]
fn derive_debug_custom_trait_test() {
    let origin = Point { x: 0, y: 0 };

    // The {:?} format specifier is used to print an instance of a type using the Debug trait.
    // println!("The origin is: {:?}", origin);
    // The {:#?} format specifier is used to print an instance of a type using the Debug trait with pretty-printing.
    println!("The origin is: {:#?}", origin);
    // Without the #[derive(Debug)] attribute, trying to print the struct would result in a compile error, because the
    // Debug trait is not implemented for the Point type.

    assert!(format!("{:?}", origin) == "Point { x: 0, y: 0 }", "debug trait");
    assert!(
        format!("{:#?}", origin) == "Point {\n\tx: 0,\n\ty: 0,\n}",
        "debug trait"
    );
}
