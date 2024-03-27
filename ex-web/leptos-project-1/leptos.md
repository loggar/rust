# leptos

Reference: https://github.com/ezesundayeze/leptos-example

```
$ cargo install trunk && cargo init leptos-project
```

```
$ cd leptos-project
$ cargo add leptos --features=csr
# csr feature flag: install only the necessary features for building a client-side application with Leptos. This will
# enable us to leave out any #features that are specific to the server-side functionality.
```

We'll convert this React project to leptos project:

```
git clone https://github.com/ezesundayeze/react-todo-sample-article
```

## Run my app:

```
$ trunk serve --open
```

## Issue with module `wasm32-unknown-unknown`:

```
$ trunk serve --open
2024-03-27T10:29:00.202778Z INFO 🚀 Starting trunk 0.19.1 2024-03-27T10:29:00.206813Z INFO 📦 starting build Compiling proc-macro2 v1.0.79 Compiling unicode-ident v1.0.11 Compiling autocfg v1.2.0 Compiling version_check v0.9.4 Compiling once_cell v1.19.0 Compiling serde v1.0.197 Compiling cfg-if v1.0.0 Compiling wasm-bindgen-shared v0.2.92 Compiling log v0.4.21 Compiling bumpalo v3.15.4 error[E0463]: can't find crate for core | = note: the wasm32-unknown-unknown target may not be installed = help: consider downloading the target with rustup target add wasm32-unknown-unknown

error[E0463]: can't find crate for compiler_builtins

For more information about this error, try rustc --explain E0463. error: could not compile cfg-if (lib) due to 2 previous errors warning: build failed, waiting for other jobs to finish... 2024-03-27T10:29:01.413582Z ERROR ❌ error error from build pipeline

Caused by: 0: HTML build pipeline failed (1 errors), showing first 1: error from asset pipeline 2: running cargo build 3: error during cargo build execution 4: cargo call to executable 'cargo' with args: '["build", "--target=wasm32-unknown-unknown", "--manifest-path", "/Users/charly.lee/ws-loggar/rust/ex-web/leptos-project-1/Cargo.toml"]' returned a bad status: exit status: 101 2024-03-27T10:29:01.414590Z INFO 📡 serving static assets at -> / 2024-03-27T10:29:01.414638Z INFO 📡 server listening at: 2024-03-27T10:29:01.414640Z INFO 🏠 http://127.0.0.1:8080/ 2024-03-27T10:29:01.414642Z INFO 🏠 http://[::1]:8080/
```

Resolve:

```
# upgrade rust if needed: $ rustup update
$ rustup target add wasm32-unknown-unknown
```
