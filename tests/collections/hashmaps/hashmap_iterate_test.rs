use std::collections::HashMap;

#[test]
fn hashmap_iterate_simple_test() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let mut result = String::new();
    let mut sum = 0;

    for (key, value) in &scores {
        result.push_str(key);
        sum += value;
    }

    assert!(
        "YellowBlue" == result || "BlueYellow" == result,
        "iterate key value pairs of hashmap"
    );
    assert_eq!(60, sum, "iterate key value pairs of hashmap");
}
