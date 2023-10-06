// A function we defined in Listing 4-9 that compiled without lifetime annotations,
// even though the parameter and return type are references
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_with_lifetime<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

#[test]
fn lifetime_elision() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_word = first_word(&novel);

    assert_eq!(
        "Call", first_word,
        "compiled without lifetime annotations, even though the parameter and return type are references"
    );

    let first_word_2 = first_word_with_lifetime(&novel);
    assert_eq!(
        "Call", first_word_2,
        "compiled with lifetime annotations, we don't need to have it"
    );
}
