use lib_root::lib_types::get_type_of;

struct AlwaysEqual;

#[test]
fn use_tuple_struct_test() {
    let subject = AlwaysEqual;

    assert_eq!(
        "types_tests::types::structs::struct_4_unit_like_struct_without_field_test::AlwaysEqual",
        get_type_of(&subject),
        "get_type_of check"
    );
}
