# manual_type_annotations_in_let_statements

### What it does

Warns on manual type annotations in `let` statements.

### Why is this bad?

In many cases, Rust can infer the type of a variable from the right-hand side of a `let`
statement. Adding an explicit type annotation in such cases is redundant and can make the
code less readable. Sometimes, type annotations are necessary, however, in those cases,
prefer to add type annotations to the right-hand side expression instead.

### Example

```rust,ignore
let x: usize = 5;
let y: Vec<i32> = [1, 2, 3].iter().copied().collect();
```

Use instead:

```rust,ignore
let x = 5usize;
let y = [1, 2, 3].iter().copied().collect::<Vec<i32>>();
```
