# blank_lines_between_variants_or_fields

### What it does

Warns on blank lines between consecutive enum variants, struct fields, union
fields, or variant fields.

### Why is this bad?

Variants and fields belong to a single declaration and should be visually
grouped. Blank lines inside a struct/enum body add vertical noise without
communicating structure. A blank line is only justified when separating
logically independent sections — which is a sign the type should be split.

Comments and doc comments between items are ignored: only physically blank
lines trigger the lint.

### Example

```rust,ignore
struct Config {
    timeout: u64,

    label: String,
}

enum Stage {
    Start,

    Middle,
    End,
}
```

Use instead:

```rust,ignore
struct Config {
    timeout: u64,
    label: String,
}

enum Stage {
    Start,
    Middle,
    End,
}
```
