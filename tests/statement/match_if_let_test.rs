#[test]
fn match_option_simple_test() {
    let some_u8_value = Some(0u8);

    let b = match some_u8_value {
        Some(3) => true,
        _ => false,
    };

    assert!(!b, "match default case");
}

#[test]
fn if_let_option_simple_test() {
    let some_u8_value = Some(0u8);

    let mut b = false;
    if let Some(3) = some_u8_value {
        b = true;
    }

    assert!(!b, "match default case");
}

#[test]
fn match_enum_simple_test() {
    let coin = Coin::Quarter(UsState::Alaska);

    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    assert_eq!(0, count, "match enum case");
}

#[test]
fn if_let_enum_simple_test() {
    let coin = Coin::Quarter(UsState::Alaska);

    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

    assert_eq!(0, count, "match enum case");
}

#[derive(Debug)]
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
