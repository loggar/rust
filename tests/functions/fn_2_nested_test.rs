fn outside_fn() -> i32 {
    fn process_data() -> i32 {
        return 31;
    }

    return process_data();
}

#[test]
fn nested_function_simple_test() {
    let r = outside_fn();

    assert_eq!(31, r, "function returns");
}
