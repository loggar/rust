#[test]
fn tuple_simple_test() {
    let immutable: (String, u8) = (String::from("StrValue"), 120);
    println!("{} {}", immutable.0, immutable.1);

    assert_eq!("StrValue", immutable.0, "immutable {} check", 0);

    assert_eq!(120, immutable.1, "immutable.{} should be {}", 1, 120);

    // immutable.1 = 142; // cannot assign

    let mut mutable = (String::from("StrValue"), 120);
    mutable.1 = 142;

    assert_eq!(142, mutable.1, "mutable.{} should be {}", 1, 142);
}

#[test]
fn tuple_destruct_test() {
    let mut mutable = (String::from("StrValue"), 120);
    let (item0, item1) = mutable;
    mutable.1 = 142;

    assert_eq!("StrValue", item0, "item0 should be {}", "StrValue");
    assert_eq!(120, item1, "item1 should be {}", 120);
    assert_eq!(142, mutable.1, "mutable.{} should be {}", 1, 142);
}
