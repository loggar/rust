#[test]
fn loop_while_test() {
    let mut result = 0;

    let numbers = [1, 2, 3, 4, 5]; // Define an array of numbers

    for num in numbers {
        result += num;
    }

    assert_eq!(15, result);
}
