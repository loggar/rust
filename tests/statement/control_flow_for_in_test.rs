#[test]
fn control_in_a_collection() {
    let mut count = 0;
    let a = [10, 20, 30, 40, 50];

    for _element in a {
        count += 1;
    }

    assert_eq!(5, count, "count should equal to {}", 5);
}

#[test]
fn control_in_a_collection_range() {
    let mut ele = 0;

    for number in 1..4 {
        println!("{number}");
        ele = number;
    }

    assert_eq!(3, ele, "ele should equal to {}", 3);

    // reverse order
    for number in (1..4).rev() {
        println!("_{number}");
        ele = number;
    }

    assert_eq!(1, ele, "ele should equal to {}", 1);
}
