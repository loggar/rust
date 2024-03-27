# cargo workspace

`Cargo.toml`

```
[workspace]

members = [
    "subproject1",
    "subproject2",
]
```

```
$ cargo new --bin subproject1
     Created binary (application) `subproject1` package
$ cargo new subproject2 --lib
     Created library `subproject2` package
```

```
├── Cargo.lock
├── Cargo.toml
├── subproject1
│   ├── Cargo.toml
│   └── src
│       └── main.rs
├── subproject2
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
└── target
```

## Dependencies between workspaces

`subproject1/Cargo.toml`

```
[dependencies]
subproject2 = { path = "../subproject2" }
```

`subproject1/src/main.rs`

```rs
use subproject2;
```

## Run

```
$ cargo run -p subproject1
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/subproject1`
Hello, world! subproject2::add(10, 10) is 20!
```

## Test

```
$ cargo test -p subproject2
```
