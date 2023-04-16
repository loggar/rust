pub fn add_two_int(a: i32, b: i32) -> i32 {
    a + b
}

// $ cargo test --test simple_mod_test

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add_two_int(2, 3), 5);
        assert_eq!(add_two_int(-1, 1), 0);
        assert_eq!(add_two_int(0, 0), 0);
    }
}
