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

// $ cargo test --test lib_tests

mod trait_ex {
    mod trait_1_simple_test;
    mod trait_2_impl_test;
    mod trait_3_function_test;
    mod trait_4_bound_test;
    mod trait_5_bound_largest_test;
}
