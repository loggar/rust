pub fn sum_even_numbers_in_range(start: u32, end: u32) -> u32 {
    (start..=end).filter(|&n| n % 2 == 0).sum()
}

pub mod simple_mod {
    #[allow(dead_code)]
    pub fn some() -> i8 {
        0
    }
}

// Marked with #[cfg(test)], will only be compiled and executed when running
// cargo test. The test function test_add will not be included in the final
// binary when you build or run the application using cargo build or cargo run.
#[cfg(test)]
mod lib_simple_tests {
    use super::*;

    #[test]
    fn test_sum_even_numbers_in_range() {
        let start = 1;
        let end = 10;
        let expected_sum = 2 + 4 + 6 + 8 + 10;
        let result = sum_even_numbers_in_range(start, end);
        assert_eq!(
            result, expected_sum,
            "Sum of even numbers in range {}..={} should be {}",
            start, end, expected_sum
        );
    }

    #[test]
    fn some_works() {
        let i = super::simple_mod::some();
        assert_eq!(0, i);
    }
}

// lib.rs: pub mod lib_simple
// $ cargo test --lib lib_simple
