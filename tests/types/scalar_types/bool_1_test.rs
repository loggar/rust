use lib_root::lib_types::get_type_of;

#[test]
fn number_int_test() {
    let t = true;
    let f: bool = false; // with explicit type annotation

    assert_eq!("bool", get_type_of(&t), "get_type_of({}) should equal to {}", t, "bool");
    assert_eq!("bool", get_type_of(&f), "get_type_of({}) should equal to {}", f, "bool");
}
