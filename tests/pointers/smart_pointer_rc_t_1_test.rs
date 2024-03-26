enum List {
    Cons(i32, Box<List>),
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

use List::{Cons, Nil};

#[test]
fn without_rc_this_not_allowed() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a)); // use of moved value: `a`

    // assert b's length
    assert_eq!(b.len(), 3);

    let mut current = &b;
    let mut sum = 0;
    loop {
        match current {
            Cons(value, tail) => {
                sum += value;
                current = tail;
            }
            Nil => break,
        }
    }
    assert_eq!(sum, 18);
}
