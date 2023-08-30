use std::collections::HashMap;

#[test]
fn hashmap_overwriting_test() {
    let mut scores = HashMap::new();

    // add values
    scores.insert(String::from("Blue"), 5);
    // overwrite a value
    scores.insert(String::from("Blue"), 10);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    assert_eq!(Some(&10), score, "create new hashmap and insert/get items from it");
}

#[test]
fn hashmap_add_only_not_present_test() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    let team_name1 = String::from("Yellow");
    let team_name2 = String::from("Blue");

    let y = scores.get(&team_name1);
    let b = scores.get(&team_name2);

    assert_eq!(Some(&50), y, "add new hashmap value when key is not present");
    assert_eq!(Some(&10), b, "add new hashmap value when key is not present");
}

#[test]
fn hashmap_update_values_based_on_old_values_test() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    //println!("map: {:?}", map);

    let k1 = "hello";
    let k2 = "world";

    let v1 = map.get(&k1);
    let v2 = map.get(&k2);

    assert_eq!(Some(&1), v1, "update value based on old value");
    assert_eq!(Some(&2), v2, "update value based on old value");
}
