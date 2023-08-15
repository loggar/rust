#[test]
fn vector_enum_to_store_multiple_types_test() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    assert_eq!(3, row.len(), "length of an enum vector");
}
