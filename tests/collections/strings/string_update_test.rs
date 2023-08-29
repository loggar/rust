#[test]
fn string_update_test() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);

    assert_eq!("foobar".to_string(), s1, "string updated by push_str");
    assert_eq!("bar".to_string(), s2, "immutable string");

    s1.push_str(" literal");
    assert_eq!("foobar literal".to_string(), s1, "string updated by push_str");
}

#[test]
fn string_update_by_plus_operator_test() {
    let s1 = String::from("foo,");
    let s2 = String::from(" bar.");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    assert_eq!(" bar.".to_string(), s2, "string used as an + operand");
    assert_eq!(
        "foo, bar.".to_string(),
        s3,
        "string where the ownership has been moved to"
    );

    // assert_eq!("foo,".to_string(), s1, "string moved by using plus operator");
    // borrow of moved value: `s1`
    // value borrowed here after move

    // why &s2?
    // the function signature of the plus operator:
    // fn add(self, s: &str) -> String {
}

#[test]
fn string_update_by_plus_operator_multiple_test() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    assert_eq!("tic-tac-toe".to_string(), s, "tic-tac-toe");
    assert_eq!("tac".to_string(), s2, "string used as an + operand");
    assert_eq!("toe".to_string(), s3, "string used as an + operand");

    // assert_eq!("foo,".to_string(), s1, "string moved by using plus operator");
    // borrow of moved value: `s1`
    // value borrowed here after move

    // why &s2?
    // the function signature of the plus operator:
    // fn add(self, s: &str) -> String {
}

#[test]
fn string_update_by_format_macro() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    assert_eq!("tic-tac-toe".to_string(), s, "tic-tac-toe");
    assert_eq!("tic".to_string(), s1, "ownership stays");
    assert_eq!("tac".to_string(), s2, "ownership stays");
    assert_eq!("toe".to_string(), s3, "ownership stays");
}
