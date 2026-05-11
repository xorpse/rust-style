# unnecessary_qualified_type_paths

### What it does

Warns on fully qualified or crate-root qualified type paths in explicit type syntax.

### Why is this bad?

Using long type paths in signatures and other type positions makes code harder to read and
maintain. In most cases, the type should be imported directly, renamed on import, or shortened
to a single disambiguating module such as `io::Result`.

### Example

```rust,ignore
fn open(path: std::path::PathBuf) -> std::io::Result<std::fs::File> {
    todo!()
}
```

Use instead:

```rust,ignore
use std::fs::File;
use std::io;
use std::path::PathBuf;

fn open(path: PathBuf) -> io::Result<File> {
    todo!()
}
```
