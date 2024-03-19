// $ cargo test --test lib_tests

mod lib_tests {
    // use lib_root::*;
    use lib_root::lib_simple::sum_even_numbers_in_range;

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
