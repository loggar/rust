#[test]
fn compare_two_strings() {
    let string1 = String::from("hello");
    let string2 = String::from("hello");
    assert_eq!(string1, string2);
}

#[test]
fn compare_two_str_types() {
    let string1 = String::from("hello");
    let string2 = String::from("hello");
    assert_eq!(string1, string2);
}

#[test]
fn compare_string_and_str_types() {
    let string: String = String::from("hello");
    let str_slice: &str = "hello";
    assert_eq!(string, str_slice);
}

#[test]
fn compare_string_case_sensitive() {
    let string1 = String::from("hello");
    let string2 = String::from("HELLO");
    assert_ne!(string1, string2); // not equal
}

#[test]
fn compare_string_case_insensitive() {
    let string1 = String::from("hello");
    let string2 = String::from("HELLO");
    assert_eq!(string1.to_lowercase(), string2.to_lowercase());
}
