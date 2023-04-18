# rust settings

## vscode

extensions:

```
rust-analyzer
CodeLLDB
```

## formatting rules

Install `rustfmt`: the official Rust code formatter

```
$ rustup component add rustfmt
```

`/.rustfmt.toml`

```
max_width = 120
```

vscode settings

```
{
    "[rust]": {
        "editor.defaultFormatter": "rust-lang.rust-analyzer"
    },
    "rust-analyzer.rustfmt.extraArgs": [
        "--config-path=${workspaceFolder}/.rustfmt.toml"
    ],
}
```

Run `rustfmt` for all `rs` files

```
$ find . -type f -name "*.rs" -exec rustfmt {} \;

> Get-ChildItem -Path . -Filter *.rs -Recurse | ForEach-Object { rustfmt $_.FullName }
```
