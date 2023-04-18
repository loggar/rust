use lib_root::lib_types::get_type_of;

#[test]
fn compound_array_test() {
    let a1 = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a2: [i32; 5] = [1, 2, 3, 4, 5];
    let a3 = [3; 5];

    assert_eq!(
        "[i32; 5]",
        get_type_of(&a1),
        "get_type_of({:?}) should equal to {}",
        a1,
        "[i32; 5]"
    );
    assert_eq!(12, months.len(), "months.len() should equal to {}", 12);
    assert_eq!(5, a2.len(), "a2.len() should equal to {}", 5);
    assert_eq!(1, a2[0], "a2[0] should equal to {}", 1);
    assert_eq!([3, 3, 3, 3, 3], a3, "a3 should equal to {:?}", [3, 3, 3, 3, 3]);
}
