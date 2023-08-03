#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
    // #[derive(Debug)]
    // error[E0277]: `Rectangle` doesn't implement `Debug`
    // = help: the trait `Debug` is not implemented for `Rectangle`
    // = note: add `#[derive(Debug)]` to `Rectangle` or manually `impl Debug for Rectangle`
}

fn print_struct() -> Rectangle {
    let r = Rectangle { width: 30, height: 50 };

    // println!("r is {}", r);
    // error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
    // = help: the trait `std::fmt::Display` is not implemented for `Rectangle`
    // = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead

    println!("r is {:?}", r);
    return r;
}

fn dbg_struct() -> Rectangle {
    let scale = 2;
    let r = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&r);
    return r;
}

#[test]
fn print_struct_test() {
    let r1 = print_struct();
    assert_eq!(50, r1.height, "struct check");

    let r2 = dbg_struct();
    assert_eq!(60, r2.width, "struct check");
}
