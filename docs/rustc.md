# `rustc`

## Example

Build and Run:

```
rustc ./ex-apps/hello/hello.rs --out-dir ./target.ex-apps && ./target.ex-apps/hello
```

## Attributes

This command will list all the configuration options that the Rust compiler provides by default for the current target.

```
$ rustc --print cfg

debug_assertions
panic="unwind"
target_arch="aarch64"
target_endian="little"
target_env=""
target_family="unix"
target_feature="aes"
target_feature="crc"
target_feature="dit"
target_feature="dotprod"
target_feature="dpb"
target_feature="dpb2"
target_feature="fcma"
target_feature="fhm"
target_feature="flagm"
target_feature="fp16"
target_feature="frintts"
target_feature="jsconv"
target_feature="lor"
target_feature="lse"
target_feature="neon"
target_feature="paca"
target_feature="pacg"
target_feature="pan"
target_feature="pmuv3"
target_feature="ras"
target_feature="rcpc"
target_feature="rcpc2"
target_feature="rdm"
target_feature="sb"
target_feature="sha2"
target_feature="sha3"
target_feature="ssbs"
target_feature="vh"
target_has_atomic="128"
target_has_atomic="16"
target_has_atomic="32"
target_has_atomic="64"
target_has_atomic="8"
target_has_atomic="ptr"
target_os="macos"
target_pointer_width="64"
target_vendor="apple"
unix
```

## Print

The rustc `--print` command can be used to print various pieces of information about the Rust compiler and its environment. Here are some of the different things you can print with rustc `--print`:

target-list `rustc --print target-list`: Prints a list of all the compilation targets supported by the installed version of rustc.

target-features `rustc --print target-features --target=<target>`: Prints the features available for a specific target.

target-cpus `rustc --print target-cpus --target=<target>`: Prints the CPUs available for a specific target.

relocation-models `rustc --print relocation-models`: Prints the relocation models supported by the compiler.

code-models `rustc --print tls-models`: Prints the code models supported by the compiler.

tls-models `rustc --print tls-models`: Prints the thread-local storage (TLS) models supported by the compiler.

link-args `rustc --print link-args`: Prints the linker arguments used by the compiler.

crate-name `rustc --print crate-name <file.rs>`: Prints the name of the crate for a given file.

sysroot `rustc --print sysroot`: Prints the sysroot path used by the compiler.

native-static-libs `rustc --print native-static-libs`: Prints the native static libraries linked to the crate.

file-names `rustc --print file-names <file.rs>`: Prints the file names produced by the compiler for a given crate.

cfg `rustc --print cfg`: Prints the configuration options that are implicitly provided by rustc for the current target.

These commands provide useful information about the Rust compiler's environment, supported targets, and various configurations. You can use them to better understand and control the compilation process.
