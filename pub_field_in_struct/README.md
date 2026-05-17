# pub_field_in_struct

### What it does

Warns on `pub` fields of named structs, tuple structs, and unions.

### Why is this bad?

Exposing fields directly makes a type's invariants impossible to enforce and ties
callers to the field layout. Prefer the project's accessor style so the
representation can change without breaking downstream code:

- getter: `fn timeout(&self) -> …`
- mutator: `fn set_timeout(&mut self, …)`
- builder: `fn with_timeout(mut self, …) -> Self` — should delegate to `set_timeout`
  so any invariants enforced by the mutator are also applied when building.

For newtype wrappers, expose an `into_inner` / accessor pair rather than `pub`
on the positional field.

Restricted visibilities (`pub(crate)`, `pub(super)`, `pub(in path)`) are not flagged —
only fully-public fields trigger this lint. `#[repr(C)]` FFI structs and similar cases
where public fields are intentional can opt out with `#[allow(pub_field_in_struct)]`.
Enum variant fields are also unaffected.

### Example

```rust,ignore
pub struct Config {
    pub timeout: u64,
}

pub struct Wrapper(pub Inner);
pub struct Inner;
```

Use instead:

```rust,ignore
pub struct Config {
    timeout: u64,
}

impl Config {
    pub fn timeout(&self) -> u64 { self.timeout }
    pub fn set_timeout(&mut self, value: u64) { self.timeout = value; }
    pub fn with_timeout(mut self, value: u64) -> Self {
        self.set_timeout(value);
        self
    }
}

pub struct Wrapper(Inner);
pub struct Inner;

impl Wrapper {
    pub fn inner(&self) -> &Inner { &self.0 }
    pub fn into_inner(self) -> Inner { self.0 }
}
```
