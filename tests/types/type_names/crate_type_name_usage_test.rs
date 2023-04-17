use typename::TypeName;

#[test]
fn simple_type_name_test() {
    let expected: &str = "std::string::String";

    assert_eq!(
        expected,
        String::type_name(),
        "String::type_name() should be {}",
        expected
    );

    println!("Vec::<i32>::type_name() {}", Vec::<i32>::type_name());
    println!("[0, 1, 2].type_name_of() {}", [0, 1, 2].type_name_of());

    assert_eq!(
        "std::vec::Vec<i32>",
        Vec::<i32>::type_name(),
        "String::type_name() should be {}",
        "std::vec::Vec<i32>"
    );

    assert_eq!(
        "[i32; 3]",
        [0, 1, 2].type_name_of(),
        "String::type_name() should be {}",
        "[i32; 3]"
    );
}
