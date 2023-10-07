pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn add_two_int(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod sample_tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
        assert_ne!(0, add_two(2)); // not equals
    }

    #[test]
    fn test_add_two_int() {
        assert_eq!(add_two_int(2, 3), 5);
        assert_eq!(add_two_int(-1, 1), 0);
        assert_eq!(add_two_int(0, 0), 0);
    }
}

pub fn greeting(name: &str) -> String {
    return format!("Hello! {}", name);
}

#[test]
fn adding_failure_messages() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{}`",
        result
    );
}

// $ cargo test --test _sample_test
