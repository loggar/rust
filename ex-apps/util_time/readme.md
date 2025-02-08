# Utils

## Dev

Build and run

```sh
cargo run
cargo run --bin tzc
cargo run --bin tsc
```

Build:

```sh
cargo build
# ./target/debug
```

Release:

```sh
cargo build --release
# ./target/release
```

Build the bin file and run (tzc):

```sh
rustc ./ex-apps/util_time/src/timezone_converter.rs -o ./target.ex-apps/util_time/tzc &&\
    ./target.ex-apps/util_time/tzc
```

Debug a binary (tzc):

```sh
$ cargo build --release
# error!
$ lldb ./target/release/tzc

(lldb) target create "./target/release/tzc"
warning: (arm64) /Users/charly.lee/ws-loggar/rust/ex-apps/util_time/target/release/tzc load command 3 LC_SEGMENT has a fileoff + filesize (0x0000000000041000) that extends beyond the end of the file (4:x16), the segment will be truncated to match
Current executable set to '/Users/charly.lee/ws-loggar/rust/ex-apps/util_time/target/release/tzc' (arm64).
```

## Timestamp converter

Run:

```sh
cargo run

cargo run --bin tsc -- 1672531200 Australia/Sydney
cargo run --bin tsc -- 1672531200
```

Run the binary:

```sh
./target/release/tsc -- 1672531200 Australia/Sydney
./target/release/tsc -- 1672531200
```

## Timezone converter

Run:

```sh
cargo run

cargo run --bin tzc -- UTC "2023-01-01 12:00:00" Australia/Sydney
cargo run --bin tzc -- UTC "2023-01-01 12:00:00"
```

Run the binary:

```sh
./target/release/tzc -- UTC "2023-01-01 12:00:00" Australia/Sydney
./target/release/tzc -- UTC "2023-01-01 12:00:00"
```
