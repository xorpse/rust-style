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
      "blank_lines_between_variants_or_fields",
      "import_grouping_and_ordering",
      "manual_type_annotations_in_let_statements",
      "nested_path_in_use_group",
      "pub_field_in_struct",
      "sorted_enum_variants",
      "thiserror_conventions",
      "to_string_on_string_types",
      "unnecessary_qualified_type_paths"
    ] }
]
```

## Available lints

- `blank_lines_between_variants_or_fields` — no blank lines between consecutive enum variants, struct fields, or union fields (comments and doc comments between items are fine).
- `import_grouping_and_ordering` — `use` declarations must be grouped as stdlib → external → same-crate, with a blank line between groups.
- `manual_type_annotations_in_let_statements` — `let x: T = expr` annotations should move to the right-hand side (turbofish, literal suffix) or rely on inference.
- `nested_path_in_use_group` — `use a::{b::c, d::e}` style brace-nested multi-segment paths should be flattened to one `use` per leaf.
- `pub_field_in_struct` — `pub` fields on structs and unions should be replaced with getter/`set_XXX`/`with_XXX` accessors.
- `sorted_enum_variants` — enum variants must be declared in case-insensitive alphabetical order; enums with explicit discriminants are exempt.
- `thiserror_conventions` — `pub` error types must not be named `Error`; `*Error` types must derive `thiserror::Error`; complex variants without `#[from]` need a snake-case constructor.
- `to_string_on_string_types` — `.to_string()` on `&str`/`String` should be `.to_owned()`.
- `unnecessary_qualified_type_paths` — fully qualified type paths like `std::path::PathBuf` in signatures should be imported or shortened.

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
