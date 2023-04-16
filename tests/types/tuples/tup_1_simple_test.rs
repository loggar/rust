#[test]
fn tuple_simple_test() {
    let immutable: (String, u8) = (String::from("StrValue"), 120);
    println!("{} {}", immutable.0, immutable.1);

    assert_eq!(immutable.0, "StrValue", "immutable {} check", 0);

    assert_eq!(immutable.1, 120, "immutable.{} should be {}", 1, 120);

    // immutable.1 = 142; // cannot assign

    let mut mutable = (String::from("StrValue"), 120);
    mutable.1 = 142;

    assert_eq!(mutable.1, 142, "mutable.{} should be {}", 1, 142);
}

#[test]
fn tuple_destruct_test() {
    let mut mutable = (String::from("StrValue"), 120);
    let (item0, item1) = mutable;
    mutable.1 = 142;

    assert_eq!(item0, "StrValue", "item0 should be {}", "StrValue");
    assert_eq!(item1, 120, "item1 should be {}", 120);
    assert_eq!(mutable.1, 142, "mutable.{} should be {}", 1, 142);
}
