# cargo

## Build

```
# Create a new cargo project
$ cargo new rust_project

# Build the project:
$ cargo build

# Build release: -> ./target/release/
$ cargo build --release
```

### profile

`Cargo.toml` default profile values:

```
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```
