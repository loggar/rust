#[test]
fn string_slice_test() {
    let s = String::from("hello");

    let slice = &s[0..2];
    println!("&s[0..2] {}", slice);

    let slice = &s[..2];
    println!("&s[..2] {}", slice);

    let len = s.len();
    println!("&s[3..len] {}", &s[3..len]);
    println!("&s[3..] {}", &s[3..]);

    println!("&s[0..len] {}", &s[0..len]);
    println!("&s[..] {}", &s[..]);

    let expected = "he";
    assert_eq!(expected, &s[..2], "&s[..2] should equal to {}", expected);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

#[test]
fn first_word_test() {
    #[allow(unused_mut)]
    let mut s = String::from("hello world");

    let word = first_word(&s);

    // s.clear(); // error!
    /*
    error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
      --> tests/types/slices/slice_1_string_test.rs:40:5
       |
    38 |     let word = first_word(&s);
       |                           -- immutable borrow occurs here
    39 |
    40 |     s.clear(); // error!
       |     ^^^^^^^^^ mutable borrow occurs here
    41 |
    42 |     println!("the first word is: {}", word);
       |                                       ---- immutable borrow later used here
    */

    println!("the first word is: {}", word);
    assert_eq!("hello", word, "word should equal to {}", "hello");
}

// A more experienced Rustacean would write the signature shown in Listing 4-9 instead because it allows us to use the
// same function on both &String values and &str values.
fn first_word2(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

#[test]
fn first_word_slice_param_test() {
    let my_string = String::from("hello world");

    let word1 = first_word2(&my_string);

    // `first_word` works on slices of `String`s, whether partial or whole
    let _word = first_word2(&my_string[0..6]);
    let word2 = first_word2(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let _word = first_word2(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let _word = first_word2(&my_string_literal[0..6]);
    let _word = first_word2(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word3 = first_word2(my_string_literal);

    assert_eq!("hello", word1, "word1 should equal to {}", "hello");
    assert_eq!("hello", word2, "word2 should equal to {}", "hello");
    assert_eq!("hello", word3, "word3 should equal to {}", "hello");
}
