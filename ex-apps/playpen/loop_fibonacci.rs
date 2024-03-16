use std::io;

fn main() {
    'outer_loop: loop {
        println!("Please input your index.");

        let mut index = String::new();

        io::stdin().read_line(&mut index).expect("Failed to read line");

        // convert the index to an integer, catch errors
        let index: u8 = match index.trim().parse::<u8>() {
            Ok(num) => num,
            Err(_) => {
                println!("Your index must be a number less than 187...");
                continue;
            }
        };

        // exit if we're gonna get too big got a u128
        if index >= 187 {
            println!("Your index must be a number less than 187...");
            continue;
        }

        let mut p: u128 = 0; // previous number
        let mut c: u128 = 0; // current number

        // Loop through all the numbers until we reach the index
        for item in 0..=index {
            let _p: u128 = p;
            p = c;
            c += _p;

            // for the first item give it a little nudge
            if c == 0 && item == 1 {
                c += 1;
            }

            // when we reach the index, print the number
            if item == index {
                println!("The fibonacci number at index {index} is {c}");
                break 'outer_loop;
            }
        }
    }
}
