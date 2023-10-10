# cli-greprs

## Create a project

```
cargo new --bin cli-greprs
```

## Run

```
cargo run searchstring example-file_path.txt
cargo run -- to in/poem.txt
IGNORE_CASE=0 cargo run -- to in/poem.txt

cargo run -- to > out/output.txt
cargo run -- to in/poem.txt out/output.txt
IGNORE_CASE=0 cargo run -- to in/poem.txt > out/output.txt
```
