#[test]
fn shadow_type_mismatch_test() {
    // Set the current value to 'a'
    let mut current_value: char = 'a';
    assert_eq!('a', current_value, "x should be now {}", 'a');

    // Set current value to 'b'
    current_value = 'b';
    assert_eq!('b', current_value, "x should be now {}", 'b');

    // 'Shadow' current_value to create a new variable
    let current_value: String = String::from("Hello, World");
    assert_eq!("Hello, World", current_value, "x should be now {}", "Hello, World");

    // Attempt to set current_value to another string
    // current_value = String::from("Oh no!!!"); // Throws an error
}
