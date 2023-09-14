struct Point<T> {
    x: T,
    y: T,
}

#[test]
fn simple_struct_with_generics() {
    let i = Point { x: 5, y: 10 };
    let f = Point { x: 1.0, y: 4.0 };
    // let wont_work = Point { x: 5, y: 4.0 };

    assert_eq!(5, i.x, "struct field with generic");
    assert_eq!(4.0, f.y, "struct field with generic");
}

struct Point2<T, U> {
    x: T,
    y: U,
}

#[test]
fn simple_struct_with_two_generics() {
    // you can do this but not recommended
    let both_integer = Point2 { x: 5, y: 10 };
    let both_float = Point2 { x: 1.0, y: 4.0 };
    let integer_and_float = Point2 { x: 5, y: 4.0 };

    assert_eq!(5, both_integer.x, "struct field with generic");
    assert_eq!(4.0, both_float.y, "struct field with generic");
    assert_eq!(5, integer_and_float.x, "struct field with generic");
}
