use lib_root::lib_types::get_type_of;

struct Person {
    name: String,
    age: u8,
}

#[test]
fn struct_simple_test() {
    let p = Person {
        name: String::from("John Doe"),
        age: 40,
    };

    assert_eq!(
        "_t_types_tests::types::structs::struct_1_simple_test::Person",
        get_type_of(&p),
        "get_type_of check"
    );

    assert_eq!("John Doe", p.name, "p.name");
    assert_eq!(40, p.age, "p.age");
}
