# sorted_enum_variants

### What it does

Warns when an enum's variants are not declared in case-insensitive alphabetical
order.

### Why is this bad?

Alphabetical ordering makes variants easy to locate when scanning a large enum
and keeps diffs minimal when new variants are added.

### When the lint skips

Enums where ordering carries semantic meaning are excluded automatically:

- any variant has an explicit discriminant (`Variant = 1`), since the numeric
  order is the contract.

For other meaningful-order enums (e.g. `LogLevel { Trace, Debug, Info, Warn,
Error }`, FFI enums with `#[repr(C)]`, state-machine enums), silence the lint
per-item with `#[allow(sorted_enum_variants)]`.

### Example

```rust,ignore
enum Direction {
    North,
    East,
    South,
    West,
}
```

Use instead:

```rust,ignore
enum Direction {
    East,
    North,
    South,
    West,
}
```
