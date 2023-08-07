#[test]
fn match_options_test() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five: Option<i32> = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    assert_eq!(6, six.unwrap(), "match option some");
    assert_eq!(None, none, "match option none");
}
