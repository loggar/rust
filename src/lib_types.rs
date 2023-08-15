pub fn get_type_of<T>(_: &T) -> &str {
    return std::any::type_name::<T>();
}

#[test]
fn simple_get_type_of_test() {
    let var_str: String = String::from("StrValue");
    let expected: &str = "alloc::string::String";

    assert_eq!(
        expected,
        get_type_of(&var_str),
        "get_type_of({}) should be {}",
        var_str,
        expected
    );
}

// lib.rs: pub mod lib_types
// $ cargo test --lib lib_types
