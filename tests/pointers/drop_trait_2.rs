use std::cell::RefCell;
use std::rc::Rc;

// In Rust, once a value has been dropped, it is considered moved and you can no longer use it. This is enforced by
// Rust's ownership rules at compile time. Therefore, you cannot directly assert that a value has been dropped because
// any attempt to access it after it has been dropped will result in a compile error.

// However, you can indirectly test that a value has been dropped by using a Rc<RefCell<bool>> to keep track of whether
// the drop function has been called. Here's how you can do it:

// Rc is a reference-counted smart pointer in Rust. It allows you to have multiple owners of a value and automatically
// clean up the value when it is no longer needed. Rc stands for reference counting. Rc::clone(&dropped) creates a new
// reference to the Rc value, incrementing the reference count. Rc::clone does not create a deep copy of the value; it
// just increments the reference count. Rc::clone is a cheap operation because it only increments the reference count,
// not the data itself.

// RefCell is a type in Rust that enforces the borrowing rules at runtime instead of compile time. The borrow_mut
// method returns a mutable reference to the value inside the RefCell, if there are no other outstanding references.
// If there are, it will panic at runtime.
// *self.dropped.borrow_mut() = true;: The * operator is used to dereference the mutable reference returned by
// borrow_mut, allowing you to modify the actual value inside the RefCell. In this case, it's setting the value to true.

// The CustomSmartPointer struct has a data field of type String and a dropped field of type Rc<RefCell<bool>>. The
// dropped field is used to keep track of whether the drop function has been called on the CustomSmartPointer instance.
// The CustomSmartPointer struct implements the Drop trait, which allows you to define custom behaviour that should be
// executed when a value is dropped. In this case, the drop function sets the value of the dropped field to true and
// prints a message to the console.
struct CustomSmartPointer {
    data: String,
    dropped: Rc<RefCell<bool>>,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        *self.dropped.borrow_mut() = true;
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

#[test]
fn test_drop() {
    let dropped = Rc::new(RefCell::new(false));
    {
        let c = CustomSmartPointer {
            data: String::from("some data"),
            dropped: Rc::clone(&dropped),
        };
        assert!(!c.data.is_empty(), "c.data should not be empty");
        drop(c);
    }
    assert!(*dropped.borrow(), "c should have been dropped");
}
