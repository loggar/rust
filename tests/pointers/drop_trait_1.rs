// https://doc.rust-lang.org/book/ch15-03-drop.html
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

#[test]
fn deref_trait_to_run_code_on_clean_up() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    assert!(!c.data.is_empty(), "c.data should not be empty");
    assert!(!d.data.is_empty(), "c.data should not be empty");

    // see what happens when scope ends
    // Dropping CustomSmartPointer with data `other stuff`!
    // Dropping CustomSmartPointer with data `my stuff`!
}

#[test]
fn drop_a_value_early_with_std_mem_drop() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    assert!(!c.data.is_empty(), "c.data should not be empty");

    drop(c);
    // how do I assert that c has been dropped?
    // You can't. Once a value has been dropped, it is considered moved and you can no longer use it. This is enforced
    // by Rust's ownership rules at compile time. Therefore, you cannot directly assert that a value has been dropped
    // because any attempt to access it after it has been dropped will result in a compile error.

    println!("CustomSmartPointer dropped before the end of scope.");
}
