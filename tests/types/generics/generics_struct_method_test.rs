struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

struct Point2<T, U> {
    x: T,
    z: U,
}

impl<T, U> Point2<T, U> {
    fn y(&self) -> &U {
        &self.z
    }
}

impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 { x: self.x, z: other.z }
    }
}

#[test]
fn use_generics_in_struct_method_test() {
    let p = Point { x: 5, y: 10 };
    let x = *p.x();

    assert_eq!(5, x, "generics in struct method");
    assert_eq!(10, p.y, "generics in struct field");

    let p1 = Point2 { x: 5, z: 10.4 };
    assert_eq!(10.4, *p1.y(), "generics in struct method");

    let p2 = Point2 { x: "Hello", z: 'c' };
    let p3 = p1.mixup(p2);

    assert_eq!(5, p3.x, "generics in struct method - mixed");
    assert_eq!('c', p3.z, "generics in struct method - mixed");
}
