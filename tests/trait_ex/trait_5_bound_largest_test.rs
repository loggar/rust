mod bound_largest_tests {
    use std::cmp::PartialOrd;

    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    #[test]
    fn trait_bound_test() {
        let numbers = vec![34, 50, 25, 100, 65];

        let result = largest(&numbers);
        assert_eq!(100, result, "largest with trait bound");

        let chars = vec!['y', 'm', 'a', 'q'];

        let result: char = largest(&chars);
        assert_eq!('y', result, "largest with trait bound");
    }
}
