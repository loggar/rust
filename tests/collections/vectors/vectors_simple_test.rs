#[test]
fn vector_for_list_test() {
    let v1 = vec![5, 6, 7, 8];

    let mut v2: Vec<i32> = Vec::new();
    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.push(8);

    assert_eq!(v1, v2, "create and update vectors");
}

#[test]
fn read_elements_of_vectors_test() {
    let v = vec![1, 2, 3, 4, 5];

    let third_item: &i32 = &v[2];

    let third: Option<&i32> = v.get(2);
    let third_int = match third {
        Some(i) => i,
        None => &0,
    };

    let not_exist: Option<&i32> = v.get(100);
    let not_exist_int = match not_exist {
        Some(i) => *i,
        None => 0,
    };

    assert_eq!(third_item, third_int, "read elements of vectors");
    assert_eq!(0, not_exist_int, "read elements of vectors");
} // <- v goes out of scope and is freed here
  // Like any other struct, a vector is freed when it goes out of scope
  // When the vector gets dropped, all of its contents are also dropped, meaning
  // the integers it holds will be cleaned up. The borrow checker ensures that any
  // references to contents of a vector are only used while the vector itself is
  // valid.
