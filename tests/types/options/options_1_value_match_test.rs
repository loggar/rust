use lib_root::lib_types::get_type_of;

#[test]
fn options_simple_test() {
    let type_inferred = Some(12); // infers type Option<{integer}>
    let type_set: Option<u8> = Some(12); // has type Option<u8>
    let type_none: Option<u8> = None; // represents absence, type Option<u8>

    assert_eq!(
        "core::option::Option<i32>",
        get_type_of(&type_inferred),
        "get_type_of check"
    );
    assert_eq!("core::option::Option<u8>", get_type_of(&type_set), "get_type_of check");
    assert_eq!("core::option::Option<u8>", get_type_of(&type_none), "get_type_of check");

    // test underlying value is 12
    assert_eq!(12, type_inferred.unwrap(), "option value");
    // test value is Some
    assert_eq!(Some(12), type_inferred, "option value");
    // test value is None
    assert_eq!(None, type_none, "option value");
}

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
