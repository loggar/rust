enum List {
    Cons(i32, Rc<List>),
    Nil,
}

impl List {
    fn len(&self) -> u32 {
        match self {
            Cons(_, tail) => 1 + tail.len(),
            Nil => 0,
        }
    }
}

use std::rc::Rc;
use List::{Cons, Nil};

#[test]
fn with_rc_multiple_owner() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

    assert_eq!(b.len(), 3);
    assert_eq!(c.len(), 3);
}

#[test]
fn cloning_rc_t_to_increase_reference_count() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // println!("count after creating a = {}", Rc::strong_count(&a));
    assert_eq!(Rc::strong_count(&a), 1);

    let _b = Cons(3, Rc::clone(&a));
    // println!("count after creating b = {}", Rc::strong_count(&a));
    assert_eq!(Rc::strong_count(&a), 2);

    {
        let _c = Cons(4, Rc::clone(&a));
        // println!("count after creating c = {}", Rc::strong_count(&a));
        assert_eq!(Rc::strong_count(&a), 3);
    }

    // println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    assert_eq!(Rc::strong_count(&a), 2);
}
