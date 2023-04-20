#[test]
fn control_while_1_simple() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    assert_eq!(5, index, "index should equal to {}", 5);
}

#[test]
fn control_loop_2_label() {
    let mut count = 0;
    let a = [10, 20, 30, 40, 50];

    for _element in a {
        count += 1;
    }

    assert_eq!(5, count, "count should equal to {}", 5);
}
