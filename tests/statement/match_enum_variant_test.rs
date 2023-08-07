#[test]
fn match_enum_variant() {
    #[allow(dead_code)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    let c = Coin::Nickel;
    let v = value_in_cents(c);

    assert_eq!(5, v, "match enum variant");
}

#[test]
fn match_enum_struct_data_test() {
    #[derive(Debug)] // So we can inspect the state in a minute
    #[allow(dead_code)]
    enum UsState {
        Alabama,
        Alaska,
        // ... etc
    }

    #[allow(dead_code)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    let c = Coin::Quarter(UsState::Alaska);
    let v = value_in_cents(c);

    assert_eq!(25, v, "match enum variant");
}
