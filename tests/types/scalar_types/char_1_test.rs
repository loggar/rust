use lib_root::lib_types::get_type_of;

#[test]
fn number_int_test() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat: char = '😻';

    assert_eq!("char", get_type_of(&c), "get_type_of({}) should equal to {}", c, "char");
    assert_eq!("char", get_type_of(&z), "get_type_of({}) should equal to {}", z, "char");
    assert_eq!("char", get_type_of(&heart_eyed_cat), "get_type_of({}) should equal to {}", heart_eyed_cat, "char");
}
