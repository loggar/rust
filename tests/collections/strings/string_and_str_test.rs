#[test]
fn string_to_str_test() {
    let k_string = String::from("hello");

    // convert using as_str()
    let k1: &str = k_string.as_str();
    // slice a String to get a &str
    let k2: &str = &k_string[..];
    // direct assignment from a literal
    let k3 = "hello";

    assert_eq!("hello", k1, "string to &str");
    assert_eq!("hello", k2, "string to &str");
    assert_eq!("hello", k3, "direct assignment from a literal string");
}
