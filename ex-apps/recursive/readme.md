# sample recursive

```
$ cargo new recursive
$ cd recursive
$ cargo build --release
```

```
$ cd target/release
$ hyperfine -i ./recursive
```

Run:

```
cargo run --package recursive --bin recursive
```
