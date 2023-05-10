#[test]
fn int_slice_test() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];
    println!("&a[1..3] {:?}", slice);

    let expected = [2, 3];
    assert_eq!(expected, slice, "&a[1..3] should equal to {:?}", expected);
}
