#[test]
fn owner_move_test() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1); // s1 is now invalid

    println!("The length of '{}' is {}.", s2, len);
    // println!("&s1 = {:p}, &s2 = {:p}", &s1, &s2);

    assert_eq!(5, len, "len should equal to {}", 5);
    // assert_eq!("hello", s1, "s1 is no longer valid"); // borrow of moved value
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

#[test]
fn owner_reference_test() {
    let s1 = String::from("hello");

    let len = calculate_length_ref(&s1); // reference as a parameter which does not own the variable.

    println!("The length of '{}' is {}.", s1, len); // s1 is still valid, ownership was not moved.
    println!("&s1 = {:p}", &s1);
}

fn calculate_length_ref(s: &String) -> usize {
    println!("&s = {:p}", &s);
    println!("s = {}", s);
    s.len()
}

#[test]
fn owner_reference_default_immutable() {
    let s = String::from("hello");

    change_invalid(&s);
    assert_eq!("hello", s, "cannot change mutable reference's value");
}

fn change_invalid(_some_string: &String) {
    // some_string.push_str(", world");
    // error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
}

#[test]
fn owner_reference_mutable_borrow() {
    let mut s = String::from("hello");

    change_mutable(&mut s);

    assert_eq!(
        "hello, world", s,
        "function did mutate the value it borrowed by using mutable reference"
    );
}

fn change_mutable(some_string: &mut String) {
    some_string.push_str(", world");
}

#[test]
fn owner_reference_rules_one_mut_ref() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    // let r2 = &mut s; // second mutable borrow occurs here
    // error[E0499]: cannot borrow `s` as mutable more than once at a time

    // println!("{}, {}", r1, r2);
    assert_eq!("hello", r1, "mutable borrow occurs");
}

#[test]
fn owner_reference_rules_one_mut_ref_in_a_scope() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        assert_eq!("hello", r1, "mutable borrow occurs");
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
    assert_eq!("hello", r2, "mutable borrow occurs");
}

#[test]
fn owner_reference_rules_cannot_combine_mut_immutable_ref() {
    // let mut s = String::from("hello");
    let s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
                 // let r3 = &mut s; // BIG PROBLEM
                 // error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable

    // println!("{}, {}, and {}", r1, r2, r3);
    assert!(r1 == r2, "immutable reference occurs"); // r1 and r2 are used here.
}

#[test]
fn owner_reference_rules_can_combine_mut_immutable_ref() {
    let mut s = String::from("hello");

    let s_ptr = &s as *const String;

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    assert!(r1 == r2, "immutable reference occurs"); // r1 and r2 are used here.
                                                     // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    assert_eq!(s_ptr, r3, "mutable reference occurs");
}

#[test]
fn no_dangle_with_moving_ownership() {
    let s = no_dangle();

    assert_eq!("hello", s, "ownership was moved");
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s // This works without any problems. Ownership is moved out, and nothing is deallocated.
}

// fn dangle() -> &String {
//    // dangle returns a reference to a String
//    let s = String::from("hello"); // s is a new String
//    &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
// Danger!
