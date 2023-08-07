#[test]
fn match_default_test() {
    fn fn_convert(x: u8) -> &'static str {
        match x {
            1 => "one",
            3 => "three",
            5 => "five",
            7 => "seven",
            _ => "",
        }
    }

    let five = fn_convert(5);
    let two = fn_convert(2);

    assert_eq!("five", five, "match a case");
    assert_eq!("", two, "match the default");
}
