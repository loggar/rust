fn sum() -> i32 {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let mut r = 0;

    for val in v1_iter {
        r += val;
    }

    return r;
}

#[test]
fn sum_returns_sum_of_vector_values_using_iter() {
    let a = sum();

    assert_eq!(6, a, "sum of vector values");
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn test_iter_map_closure_collect() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}
