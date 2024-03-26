#[derive(Debug)]
enum Color {
    Hex(String),
    RGB(u8, u8, u8),
    HSL(u16, f32, f32),
}

impl Color {
    // Combines two colors into a new color by calculating the mean average of
    // their RGB components.
    fn combine(&self, other: &Color) -> Color {
        let (r1, g1, b1) = self.to_rgb();
        let (r2, b2, g2) = other.to_rgb();

        // Calculate the mean average of each RGB component
        let r = r1 / 2 + r2 / 2;
        let g = g1 / 2 + g2 / 2;
        let b = b1 / 2 + b2 / 2;

        Color::RGB(r, g, b)
    }

    // Converts the color to RGB format
    fn to_rgb(&self) -> (u8, u8, u8) {
        match self {
            Color::Hex(hex) => {
                let temp_color: u32 = u32::from_str_radix(hex, 16).expect("Failed to convert");
                let r = ((temp_color >> 16) & 0xFF) as u8;
                let g = ((temp_color >> 8) & 0xFF) as u8;
                let b = (temp_color & 0xFF) as u8;

                (r, g, b)
            }
            Color::RGB(r, g, b) => (r.clone(), g.clone(), b.clone()),
            Color::HSL(h, s, l) => {
                let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
                let x = c * (1.0 - ((h.clone() as f32 / 60.0) % 2.0 - 1.0).abs());
                let m = l - c / 2.0;

                let (r, g, b) = if h < &60 {
                    (c, x, 0.0)
                } else if h < &120 {
                    (x, c, 0.0)
                } else if h < &180 {
                    (0.0, c, x)
                } else if h < &240 {
                    (0.0, x, c)
                } else if h < &300 {
                    (x, 0.0, c)
                } else {
                    (c, 0.0, x)
                };

                let r = ((r + m) * 255.0).round() as u8;
                let g = ((g + m) * 255.0).round() as u8;
                let b = ((b + m) * 255.0).round() as u8;

                (r, g, b)
            }
        }
    }

    fn to_string(&self) -> String {
        match self {
            Color::Hex(hex) => format!("#{}", hex),
            Color::RGB(r, g, b) => format!("RGB - R: {}, G: {}, B: {}", r, g, b),
            Color::HSL(h, s, l) => format!("HSL - H: {}, S: {}, L: {}", h, s, l),
        }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Color::Hex(a), Color::Hex(b)) => a == b,
            (Color::RGB(r1, g1, b1), Color::RGB(r2, g2, b2)) => r1 == r2 && g1 == g2 && b1 == b2,
            (Color::HSL(h1, s1, l1), Color::HSL(h2, s2, l2)) => h1 == h2 && s1 == s2 && l1 == l2,
            _ => false,
        }
    }
}

#[test]
fn enum_discern_test() {
    let hex_tomato = Color::Hex(String::from("ff6347"));
    let color_mix = hex_tomato.combine(&Color::HSL(243, 1.0, 0.38));

    // assert to_string
    assert_eq!("#ff6347", hex_tomato.to_string());
    assert_eq!("RGB - R: 132, G: 146, B: 35", color_mix.to_string());

    // assert mix
    assert_eq!(Color::RGB(132, 146, 35), color_mix);
}
