#[test]
fn loop_while_test() {
    let mut count = 0;

    while count < 5 {
        count += 1;
    }

    assert_eq!(5, count);
}
