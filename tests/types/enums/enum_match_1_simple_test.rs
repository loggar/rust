use lib_root::lib_types::get_type_of;

#[test]
fn enum_variants_test() {
    enum Color {
        Hex,
        RGB,
        HSL,
    }

    // We now know 'a' is a hex colour but have no idea what its value actually is.
    let a = Color::Hex;
    let _b = Color::RGB;
    let _c = Color::HSL;

    assert_eq!(
        "_t_types_tests::types::enums::enum_match_1_simple_test::enum_variants_test::Color",
        get_type_of(&a),
        "get_type_of check"
    );
}

#[test]
fn enum_variants_impl_test() {
    // With this setup, we not only identify 'tomato'
    // as a hex value, but we also see the actual hex
    // value assigned to it
    #[derive(Debug)] // Add this line to implement the Debug trait for the Color enum
    enum Color {
        Hex(String),
        RGB(u8, u8, u8),
    }

    // implement equality for Color
    impl PartialEq for Color {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Color::Hex(a), Color::Hex(b)) => a == b,
                (Color::RGB(r1, g1, b1), Color::RGB(r2, g2, b2)) => r1 == r2 && g1 == g2 && b1 == b2,
                _ => false,
            }
        }
    }

    // Introduce a method to enable color mixing
    impl Color {
        fn combine(&self, other: &Color) -> Color {
            // combine two colors, self and other
            match (self, other) {
                (Color::RGB(r1, g1, b1), Color::RGB(r2, g2, b2)) => {
                    Color::RGB((r1 + r2) / 2, (g1 + g2) / 2, (b1 + b2) / 2)
                }
                _ => Color::RGB(0, 0, 0),
            }
        }
    }

    let tomato = Color::Hex(String::from("ff6347"));

    assert!(match tomato {
        Color::Hex(_) => true,
        _ => false,
    });

    // test combine function
    let red = Color::RGB(255, 0, 0);
    let green = Color::RGB(0, 255, 0);
    let yellow = red.combine(&green);
    // assert yellow is a mix of red and green
    assert_eq!(yellow, Color::RGB(127, 127, 0));
}
