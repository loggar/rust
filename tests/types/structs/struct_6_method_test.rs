#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated functions that aren’t methods are often used for constructors
    // that will return a new instance of the struct. These are often called new,
    // but new isn’t a special name and isn’t built into the language.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

#[test]
fn rectangle_area_test() {
    let r1 = Rectangle { width: 30, height: 50 };
    let r2 = Rectangle { width: 10, height: 40 };
    let r3 = Rectangle { width: 60, height: 45 };

    assert_eq!(1500, r1.area(), "method returns");
    assert!(r1.can_hold(&r2), "method returns");
    assert!(!r2.can_hold(&r3), "method returns");
}

#[test]
fn associated_function_test() {
    let sq = Rectangle::square(3);

    assert!(sq.width == sq.height, "associated function returns");
}

struct Book {
    rating: f32,
}

impl Book {
    // Method to check if the book is recommended based on its rating
    fn is_recommended(&self, star_limit: f32) -> bool {
        // Check if the rating is greater than the star_limit
        self.rating > star_limit
    }
}

const RECOMMEND_THRESHOLD: f32 = 3.5;

#[test]
fn method_call_with_args_test() {
    let book1 = Book { rating: 4.8 };

    // Call the is_recommended method on book1
    let is_recommended = book1.is_recommended(RECOMMEND_THRESHOLD);

    assert!(is_recommended, "method returns");
}
