#![allow(unused)]
enum Color {
    Hex(String),
    RGB(u8, u8, u8),
    HSL(u8, f32, f32),
}

impl Color {
    fn print(&self) {
        match self {
            // if the color is in hex, take the string as an argument
            Color::Hex(hex) => {
                println!("Hex - #{}", hex);
            }
            // if the color is RGB, take each part of the tuple
            // as arguments
            Color::RGB(r, g, b) => {
                println!("RGB - R: {}, G: {}, B: {}", r, g, b);
            }
            // if the color is HSL, take each part of the tuple
            // as arguments
            Color::HSL(h, s, l) => {
                println!("HSL - H: {}, S: {}, L: {}", h, s, l);
            }
        }
    }
}

#[test]
fn enum_discern_test() {
    let tomato = Color::Hex(String::from("ff6347"));

    // assert tomato.print()
    assert_eq!((), tomato.print());
}
