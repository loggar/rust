#[test]
fn loop_sample_test() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // Exit the loop and return the value 20
        }
    };

    assert_eq!(20, result);
}

#[test]
fn nested_loop_sample_test() {
    let mut total = 5;

    'outer_loop: loop {
        let mut iteration = 0;

        loop {
            iteration += 1;
            println!("Iteration {} with {} loops remaining.", iteration, total);

            if iteration == 5 {
                if total == 0 {
                    break 'outer_loop; // Exit the outer loop
                }
                break; // Exit the inner loop
            }
        }

        total -= 1;
    }

    assert_eq!(0, total);
}
