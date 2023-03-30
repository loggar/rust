# cargo

## commands

```
# Create a new cargo project
$ cargo new rust-project

# Build the project:
$ cargo build

# Run the project:
$ cargo run

# Update project dependencies:
$ cargo update

# Run tests:
$ cargo test

# Generate the project documentation via rustdoc:
$ cargo doc

# Analyze the project to see it has any errors, without building it:
$ cargo check
```

```
# Log in to crates.io with the API token:
$ cargo login

# Make the local crate uploadable to crates.io:
$ cargo package

# Upload the crate to crates.io:
$ cargo publish
```

## Sub program

```
$ cargo new sub_program --bin
# src/bin/sub_program.rs

$ cargo build
$ cargo run --bin sub_program
```
