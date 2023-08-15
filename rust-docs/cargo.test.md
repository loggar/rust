# cargo test

```
$ cargo test
```

## Test a part of tests

```
# tests/simple_test.rs
$ cargo test --test _simple_test
```

## Test lib in `src/`

```
$ cargo test --lib
```

```
$ cargo test --lib

    Finished test [unoptimized + debuginfo] target(s) in 0.05s
     Running unittests src/lib.rs (target/debug/deps/lib_root-dfe8d86ea91a61bb)

running 2 tests
test lib_mod_test::test_sum_even_numbers_in_range ... ok
test lib_types::simple_get_type_of_test ... ok
```
