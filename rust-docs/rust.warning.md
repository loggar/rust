# rust lint

## allow warnings

### `unused_mut`

```rs
#[allow(unused_mut)]
let mut s = String::from("hello world");
```

```
  --> tests/types/slices/slice_1_string_test.rs:37:9
   |
37 |     let mut s = String::from("hello world");
   |         ----^
   |         |
   |         help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` on by default
```
