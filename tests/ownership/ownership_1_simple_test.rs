#[test]
fn owner_simple_1_stack() {
    let s1: &str = "hello"; // s1 is valid from this point forward

    // string literal is immutable.
    // do stuff with s1
    println!("{}", s1);

    let s2: &str = s1;
    println!("{}", s2);
    println!("{}", s1);
    println!("&s1 = {:p}, &s2 = {:p}", &s1, &s2);

    assert!(s1 == s2, "s1 == s2 should be true");
    assert_eq!(s1, s2, "s1 should equal to s2");
    assert!(!std::ptr::eq(&s1, &s2), "s1, s2 pointers should not be same");
}
// this scope is now over, and s1 is no longer valid

#[test]
fn owner_simple_2_heap_alloc() {
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}, world!", s1);
    println!("{}, world!", s2);

    // assert!(s1 == s2, "s1 == s2 should be true");
    // assert_eq!(s1, s2, "s1 should equal to s2");
}
// |     let s1 = String::from("hello");
// |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
// |     let s2 = s1;
// |              -- value moved here
// |
// |     println!("{}, world!", s1);
// |                            ^^ value borrowed here after move
// |
// = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
// help: consider cloning the value if the performance cost is acceptable
// |
// |     let s2 = s1.clone();

#[test]
fn owner_simple_3_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
    println!("&s1 = {:p}, &s2 = {:p}", &s1, &s2);

    assert!(s1 == s2, "s1 == s2 should be true");
    assert_eq!(s1, s2, "s1 should equal to s2");
    assert!(!std::ptr::eq(&s1, &s2), "s1, s2 pointers should not be same");
}

#[test]
fn owner_simple_4_no_move_for_int() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    assert!(x == y, "x == y should be true");
    assert_eq!(x, y, "x should equal to y");
    assert!(!std::ptr::eq(&x, &y), "x, y pointers should not be same");

    // we don’t have a call to clone, but x is still valid and wasn’t moved into y.
    // The reason is that types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make. That means there’s no reason we would want to prevent x from being valid after we create the variable y. In other words, there’s no difference between deep and shallow copying here, so calling clone wouldn’t do anything different from the usual shallow copying, and we can leave it out.
}
