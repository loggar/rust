# Rust install/update

## install

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

```
info: downloading installer

Welcome to Rust!

This will download and install the official compiler for the Rust
programming language, and its package manager, Cargo.

Rustup metadata and toolchains will be installed into the Rustup
home directory, located at:

  /Users/<user>/.rustup

This can be modified with the RUSTUP_HOME environment variable.

The Cargo home directory is located at:

  /Users/<user>/.cargo

This can be modified with the CARGO_HOME environment variable.

The cargo, rustc, rustup and other commands will be added to
Cargo's bin directory, located at:

  /Users/<user>/.cargo/bin

This path will then be added to your PATH environment variable by
modifying the profile files located at:

  /Users/<user>/.profile
  /Users/<user>/.bash_profile
  /Users/<user>/.bashrc
  /Users/<user>/.zshenv

You can uninstall at any time with rustup self uninstall and
these changes will be reverted.

Current installation options:


   default host triple: aarch64-apple-darwin
     default toolchain: stable (default)
               profile: default
  modify PATH variable: yes

1) Proceed with installation (default)
2) Customize installation
3) Cancel installation

...

  stable-aarch64-apple-darwin installed - rustc 1.69.0 (84c898d65 2023-04-16)


Rust is installed now. Great!

To get started you may need to restart your current shell.
This would reload your PATH environment variable to include
Cargo's bin directory ($HOME/.cargo/bin).

To configure your current shell, run:
source "$HOME/.cargo/env"
```

```
. "$HOME/.cargo/env"
```

## update

```
$ rustup update stable

$ cargo --version
cargo 1.76.0 (c84b36747 2024-01-18)
```
