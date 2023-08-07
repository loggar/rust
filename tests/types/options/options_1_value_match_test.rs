use lib_root::lib_types::get_type_of;

#[test]
fn options_value_match_test() {
    let i: i8 = 5;
    let y: Option<i8> = Some(i);

    assert_eq!("core::option::Option<i8>", get_type_of(&y), "get_type_of check");

    let i1 = match y {
        Some(val) => val,
        None => 0, // or any default value you'd like
    };

    let i2 = y.unwrap();
    let i3 = y.unwrap_or(0);

    let i4 = y.unwrap_or_else(|| {
        // Do some computation...
        0 // This will be the default value
    });

    assert_eq!(i, i1, "option value");
    assert_eq!(i, i2, "option value");
    assert_eq!(i, i3, "option value");
    assert_eq!(i, i4, "option value");

    let default_value: i8 = 0;
    let x: Option<i8> = None;

    let j1 = match x {
        Some(val) => val,
        None => default_value, // or any default value you'd like
    };

    let j2: i8 = x.unwrap_or(default_value);

    assert_eq!(default_value, j1, "option value");
    assert_eq!(default_value, j2, "option value");
}
