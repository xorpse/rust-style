# to_string_on_string_types

### What it does

Warns on using `to_string()` on `String` or `&str` to obtain a `String` when `.to_owned()`
should be preferred, as it more clearly expresses the intent of creating an owned `String`.

### Why is this bad?

Using `.to_owned()` is generally more idiomatic when considering the intent of obtaining an
owned version of a type.

### Example

```rust
let a = "hello".to_string();
let b = "hello";
let c = &&b;

let d = c.to_string();
let e = d.to_string();
```

Use instead:

```rust
let a = "hello".to_owned();
let b = "hello";
let c = &&b;

let d = c.to_owned();
let e = d.to_owned();
```
