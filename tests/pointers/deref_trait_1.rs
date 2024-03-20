use std::ops::Deref;

// https://doc.rust-lang.org/book/ch15-02-deref.html
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[test]
fn deref_trait_to_treat_as_regular_references() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // *y is equivalent to *(y.deref())
}

// hello function returns new string with the name passed as an argument
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[test]
fn test_implicit_deref_coercion() {
    let m = MyBox::new(String::from("Rust"));

    // Calling hello with a reference to a MyBox<String> value, which works because of deref coercion
    let s = hello(&m);
    assert!(s == "Hello, Rust!", "s should be 'Hello, Rust!'");

    // If Rust didn't have deref coercion, we would have to write the following code:
    // hello(&(*m)[..]);
    // This is because (*m) would dereference the MyBox<String> into a String, and then & and [..] would take a string slice of the String
    // The deref coercion feature makes this code more concise and easier to read
}
