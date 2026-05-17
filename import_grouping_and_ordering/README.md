# import_grouping_and_ordering

### What it does

Warns when `use` declarations in a module are not grouped and ordered as
1) standard library, 2) external crates, 3) same crate, with a blank line
separating each non-empty group.

### Why is this bad?

Consistently grouping `use` declarations makes the imports section of a file
easy to scan. Mixing standard library, external crate, and same-crate imports
without separation hides relationships between modules.

The expected layout is:

1. `std`, `core`, `alloc`
2. external crates (anything else)
3. same-crate paths (`crate::`, `self::`, `super::`)

with a blank line between each non-empty group. Equivalent to running
`rustfmt --config group_imports=StdExternalCrate,imports_granularity=Module`,
but surfaced as a lint at edit time.

The lint only inspects the leading run of `use` (and `extern crate`) items in
each module — imports interleaved with other items elsewhere in the module
body are not analysed.

### Example

```rust,ignore
use serde::Serialize;
use std::collections::HashMap;
use crate::models::DataModel;
```

Use instead:

```rust,ignore
use std::collections::HashMap;

use serde::Serialize;

use crate::models::DataModel;
```
