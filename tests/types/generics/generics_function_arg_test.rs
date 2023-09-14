fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

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
fn largest_number_and_char_test() {
    let numbers = vec![34, 50, 25, 100, 65];
    let result_i = largest_i32(&numbers);

    let chars = vec!['y', 'm', 'a', 'q'];
    let result_c = largest_char(&chars);

    assert_eq!(100, result_i, "largest number");
    assert_eq!('y', result_c, "largest char");

    let result_g = largest(&chars);
    assert_eq!('y', result_g, "largest value");
}
