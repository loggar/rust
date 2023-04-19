#[test]
fn block_exp_simple_test() {
    // block expression
    let y = {
        let x = 2;
        x + 3 // no semicolon
    };

    assert_eq!(5, y, "y should equal to {}", 5);
}

fn five() -> i32 {
    // return expression
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

#[test]
fn return_exp_simple_test() {
    let x = five();
    let y = plus_one(x);

    assert_eq!(5, x, "x should equal to {}", 5);
    assert_eq!(6, y, "y should equal to {}", 6);
}
