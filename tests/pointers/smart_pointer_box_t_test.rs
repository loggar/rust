fn heap_pointer_owner() -> i32 {
    let b = Box::new(5);
    // println!("b = {}", b);
    return *b;
}

#[test]
fn smart_pointer_box_t_test() {
    let b = heap_pointer_owner();
    let i = 5;

    assert!(b == i, "b should be 5");
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

#[test]
fn smart_point_box_t_recursive_test() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // println!("1 = {}", std::mem::size_of_val(&list));
    // println!("2 = {}", std::mem::size_of::<i32>());
    // println!("3 = {}", std::mem::size_of::<List>());

    // assert the list is in the heap
    assert!(
        std::mem::size_of_val(&list) > std::mem::size_of::<i32>(),
        "list should be in the heap"
    );
}
