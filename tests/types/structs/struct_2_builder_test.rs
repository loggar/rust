use lib_root::lib_types::get_type_of;

struct User {
    username: String,
    email: String,
    #[allow(dead_code)]
    sign_in_count: u64,
    #[allow(dead_code)]
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

#[test]
fn struct_build_user_test() {
    let user_1 = build_user(String::from("someone@example.com"), String::from("someUsername123"));

    assert_eq!(
        "types_tests::types::structs::struct_2_builder_test::User",
        get_type_of(&user_1),
        "get_type_of check"
    );

    assert_eq!("someone@example.com", user_1.email, "p.name");
    assert_eq!("someUsername123", user_1.username, "p.name");
}
