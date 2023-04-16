use lib_root::lib_types::get_type_of;

type SampleTupleTestType = (String, u8);

#[test]
fn tuple_type_test() {
    let immutable: SampleTupleTestType = (String::from("StrValue"), 120);
    println!("{} {}", immutable.0, immutable.1);
    println!("{}", get_type_of(&immutable));

    assert_eq!(
        get_type_of(&immutable),
        "(alloc::string::String, u8)",
        "get_type_of {}",
        "SampleTupleTestType"
    );
}
