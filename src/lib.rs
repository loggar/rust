pub fn sum_even_numbers_in_range(start: u32, end: u32) -> u32 {
    (start..=end).filter(|&n| n % 2 == 0).sum()
}

#[cfg(test)]
mod tests {
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
}
