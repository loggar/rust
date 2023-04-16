fn add(a: i32, b: i32) -> i32 {
    a + b
}

// $ cargo test --test simple_test

#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);
    assert_eq!(add(-1, 1), 0);
    assert_eq!(add(0, 0), 0);
}
