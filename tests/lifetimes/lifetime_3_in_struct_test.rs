struct ImportantExcerpt<'a> {
    part: &'a str,
}

#[test]
fn limit_lifetime_of_struct_field() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };

    assert_eq!(
        "Call me Ishmael", i.part,
        "A struct that holds a reference, requiring a lifetime annotation"
    );
}

// The lifetime parameter declaration after impl and its use after the type name are required,
// but we’re not required to annotate the lifetime of the reference to self because of the first elision rule.
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

#[test]
fn limit_in_struct_method() {
    let i = ImportantExcerpt { part: "Call" };

    assert_eq!(3, i.level(), "A method with lifetime");
}

// There are two input lifetimes, so Rust applies the first lifetime elision rule and gives both
// &self and announcement their own lifetimes. Then, because one of the parameters is &self,
//  the return type gets the lifetime of &self, and all lifetimes have been accounted for.
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

#[test]
fn limit_in_struct_method_2() {
    let i = ImportantExcerpt { part: "Call" };

    assert_eq!("Call", i.announce_and_return_part("here"), "A method with lifetime");
}
