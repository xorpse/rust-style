# Lints for more pleasant Rust code

## Usage

```
cargo install cargo-dylint dylint-link
```

Then run the lints with:
```
cargo dylint --git https://github.com/xorpse/rust-style --pattern '*'
```

## Development usage

The lints can be configured using a `dylint.toml` file. For example:

```toml
[workspace.metadata.dylint]
libraries = [
    { git = "https://github.com/xorpse/rust-style", pattern = [
      "manual_type_annotations_in_let_statements",
      "to_string_on_string_types"
    ] }
]
```

### rust-analyzer integration

Configure the `check` command as follows (assuming we have the above `dylint.toml`
file in the crate/workspace root):

```json
"rust-analyzer.check.overrideCommand": [
    "cargo",
    "dylint",
    "--all",
    "--",
    "--all-targets",
    "--message-format=json"
]
```
