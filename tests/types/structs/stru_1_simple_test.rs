use lib_root::lib_types::get_type_of;

struct Person {
    name: String,
    age: u8,
}

#[test]
fn struct_simple_test() {
    let p = Person {
        name: String::from("Jhon Doe"),
        age: 40,
    };

    assert_eq!(
        get_type_of(&p),
        "types_tests::types::structs::stru_1_simple_test::Person",
        "get_type_of"
    );

    assert_eq!(p.name, "Jhon Doe", "p.name");
    assert_eq!(p.age, 40, "p.age");
}
