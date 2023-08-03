use lib_root::lib_types::get_type_of;

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[test]
fn use_tuple_struct_test() {
    let black: Color = Color(0, 0, 0);
    let origin: Point = Point(0, 0, 0);

    assert_eq!(
        "_t_types_tests::types::structs::struct_3_tuple_struct_test::Color",
        get_type_of(&black),
        "get_type_of check"
    );

    assert_eq!(
        "_t_types_tests::types::structs::struct_3_tuple_struct_test::Point",
        get_type_of(&origin),
        "get_type_of check"
    );
}
