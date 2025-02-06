# cargo crates

## Account

https://crates.io/

## Publish my crates

Crate Metadata

`Cargo.toml`

```
[package]
name = "app_gussing_game"
version = "0.1.0"
edition = "2021"
description = "A fun game where you guess what number the computer has chosen."
license = "MIT OR Apache-2.0"

[dependencies]
```

Publish Crate:

```sh
$ cargo publish
```

## Publishing a New Version of an Existing Crate

https://semver.org/

```sh
$ cargo publish
```

## Deprecating Versions from Crates.io

```sh
$ cargo yank --vers 1.0.1
    Updating crates.io index
        Yank app_gussing_game@1.0.1
```

By adding `--undo` to the command, you can also undo a yank and allow projects to start depending on a version again:

```sh
$ cargo yank --vers 1.0.1 --undo
    Updating crates.io index
      Unyank app_gussing_game@1.0.1
```
