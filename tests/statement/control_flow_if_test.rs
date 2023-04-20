#[test]
fn control_if_else_test() {
    let number = 6;
    let r: i32;

    if number % 4 == 0 {
        r = 4
    } else if number % 3 == 0 {
        r = 3
    } else if number % 2 == 0 {
        r = 2
    } else {
        r = 1
    }

    assert_eq!(3, r, "r should equal to {}", 3);
}

#[test]
fn control_if_in_let_test() {
    let condition = false;
    let number = if condition { 5 } else { 6 };


    assert_eq!(6, number, "number should equal to {}", 6);
}
