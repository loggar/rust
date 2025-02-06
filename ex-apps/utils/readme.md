# Utils

## Dev

Build and run

```
cargo run
cargo run --bin tzc
```

Build:

```
cargo build
./target/debug/tzc
```

Release:

```
cargo build --release
./target/release/tzc
```

## Timezone converter

Build debug:

```sh
cargo build
```

Run:

```sh
cargo run

cargo run --bin tzc -- UTC Australia/Sydney "2023-01-01 12:00:00"
```

Build the bin file and run:

```sh
rustc ./ex-apps/utils/src/timezone_converter.rs -o ./target.ex-apps/utils/tzc &&\
    ./target.ex-apps/utils/tzc
```

Debug a binary:

```sh
$ cargo build --release
# error!
$ lldb ./target/release/tzc

(lldb) target create "./target/release/tzc"
warning: (arm64) /Users/charly.lee/ws-loggar/rust/ex-apps/utils/target/release/tzc load command 3 LC_SEGMENT has a fileoff + filesize (0x0000000000041000) that extends beyond the end of the file (4:x16), the segment will be truncated to match
Current executable set to '/Users/charly.lee/ws-loggar/rust/ex-apps/utils/target/release/tzc' (arm64).
```

Run the binary:

```sh
./target/release/tzc -- UTC Australia/Sydney "2023-01-01 12:00:00"
```
