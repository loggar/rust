#[test]
fn string_new_from_test() {
    let mut _s = String::new();

    let data = "initial contents";
    let _s = data.to_string();

    // the method also works on a literal directly:
    let _s = "initial contents".to_string();

    let _s = String::from("initial contents");

    let _hello = String::from("السلام عليكم");
    let _hello = String::from("Dobrý den");
    let _hello = String::from("Hello");
    let _hello = String::from("שָׁלוֹם");
    let _hello = String::from("नमस्ते");
    let _hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let _hello = String::from("你好");
    let _hello = String::from("Olá");
    let _hello = String::from("Здравствуйте");
    let _hello = String::from("Hola");

    assert_eq!("안녕하세요".to_string(), hello, "length of an enum vector");
}
