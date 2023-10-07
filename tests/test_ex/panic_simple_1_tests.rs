pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

#[test]
#[should_panic]
fn greater_than_100() {
    let g = Guess::new(200);

    assert_eq!(4, g.value);
}
