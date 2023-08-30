use std::collections::HashMap;

#[test]
fn hashmap_new_test() {
    let mut scores = HashMap::new();

    // add values
    scores.insert(String::from("Blue"), 5);
    scores.insert(String::from("Yellow"), 50);

    // overwrite a value
    scores.insert(String::from("Blue"), 10);

    // print map
    // println!("scores: {:?}", scores);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    assert_eq!(10, *score.unwrap(), "create new hashmap and insert/get items from it");
    assert_eq!(Some(&10), score, "create new hashmap and insert/get items from it");
    assert_eq!(
        10,
        *score.unwrap_or(&0),
        "create new hashmap and insert/get items from it"
    );

    match score {
        Some(&value) => assert_eq!(10, value, "create new hashmap and insert/get items from it"),
        None => panic!("Key not found in HashMap"),
    }
}

#[test]
fn vector_to_hashmap_test() {
    let teams: Vec<String> = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores: Vec<i32> = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    assert_eq!(10, **score.unwrap(), "vectors to hashmap");
}

#[test]
fn hashmap_ownership_test() {
    // For types that implement the Copy trait, like i32, the values are copied
    // into the hash map. For owned values like String, the values will be moved
    // and the hash map will be the owner of those values

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    let team_name = String::from("Favorite color");
    let score = map.get(&team_name);

    assert_eq!(
        "Blue",
        *score.unwrap(),
        "create new hashmap and insert/get items from it"
    );

    // assert_eq!("Blue", field_value, "create new hashmap and insert/get items from it");
    // borrow of moved value: `field_value`
}
