# thiserror_conventions

### What it does

Enforces project conventions on error types built with [`thiserror`](https://docs.rs/thiserror).
A single lint covering four sub-rules:

1. **Naming.** A `pub` error type must not be called literally `Error` — it must carry
   a descriptive prefix (e.g. `ParseError`, `IoError`) so callers can `use` it
   without conflicting with `std::error::Error` or other crates' `Error` types.
2. **Variant ordering.** Variants of an error enum must be sorted alphabetically
   (case-insensitive).
3. **Mandatory derive.** Any type whose name ends in `Error` must derive
   `thiserror::Error` (`#[derive(Error)]` or `#[derive(thiserror::Error)]`).
4. **Constructor methods.** Each variant with one or more fields and without a
   `#[from]` field attribute must have a snake-case constructor
   `impl MyError { fn variant_name(...) -> Self }` somewhere in the same crate.
   Variants with `#[from]` get their constructor "for free" via `From::from`;
   unit variants need no constructor.

### Why is this bad?

These conventions keep error types predictable: callers can `use` them without
conflict, variants are easy to find in source, error rendering is uniform via
`thiserror`, and complex variants have a single canonical constructor.

### Pass type

This lint runs as a **pre-expansion** pass so that `#[derive(...)]` and `#[from]`
attributes are still visible in the AST. The constructor cross-reference checks
inherent `impl` blocks as written in source.

### Limitations

- The "must derive thiserror" check uses a type-name heuristic (`*Error`).
  False positives on incidentally-named types (e.g. `NotAnError`) can be
  silenced with `#[allow(thiserror_conventions)]`.
- Re-exports of the `Error` derive through other modules are not detected —
  only the two derive spellings `Error` and `thiserror::Error` are recognised.
- The constructor check is crate-local. Constructors defined in a different
  crate are not detected.

### Example

```rust,ignore
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("bad")]
    BadInput { reason: String },
    #[error("io")]
    A,
}
```

Use instead:

```rust,ignore
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("io")]
    A,
    #[error("bad")]
    BadInput { reason: String },
}

impl ParseError {
    pub fn bad_input(reason: impl Into<String>) -> Self {
        Self::BadInput { reason: reason.into() }
    }
}
```
