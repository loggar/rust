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
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn build_user_field_init_shorthand(email: String, username: String) -> User {
    User {
        email,    // Using the Field Init Shorthand
        username, //Using the Field Init Shorthand
        active: true,
        sign_in_count: 1,
    }
}

fn create_instances_from_other_instances(user1: User) -> User {
    User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    }
}

fn create_instances_from_other_instances_rest(user1: User) -> User {
    User {
        email: String::from("another@example.com"),
        ..user1
    }
}

#[test]
fn struct_build_user_test() {
    let user_1 = build_user(String::from("someone@example.com"), String::from("someUsername123"));

    assert_eq!(
        "_t_types_tests::types::structs::struct_2_builder_test::User",
        get_type_of(&user_1),
        "get_type_of check"
    );

    assert_eq!("someone@example.com", user_1.email, "user_1.email");
    assert_eq!("someUsername123", user_1.username, "user_1.username");

    let user_2 = build_user_field_init_shorthand(String::from("someone@example.com"), String::from("someUsername123"));
    assert_eq!("someone@example.com", user_2.email, "user_2.email");
    assert_eq!("someUsername123", user_2.username, "user_2.username");

    let user_3 = create_instances_from_other_instances(user_1);
    assert_eq!(true, user_3.active, "user_3.active");

    let user_4 = create_instances_from_other_instances_rest(user_2);
    assert_eq!(true, user_4.active, "user_4.active");
}
