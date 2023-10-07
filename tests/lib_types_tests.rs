mod lib_types_tests {
    // use lib_root::*;
    use lib_root::lib_types::get_type_of;

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
}

// $ cargo test --test lib_types_tests
