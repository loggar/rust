pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exported_function() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn test_not_exported_function() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
