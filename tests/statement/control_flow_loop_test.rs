#[test]
fn control_loop_1_simple() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(20, result, "result should equal to {}", 20);
}

#[test]
fn control_loop_2_label() {
    let mut count = 0;
    // label: 'counting_up
    'counting_up: loop {
        let mut remaining = 10;

        loop {
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    assert_eq!(2, count, "count should equal to {}", 2);
}
