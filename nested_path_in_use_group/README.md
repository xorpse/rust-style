# nested_path_in_use_group

### What it does

Warns on `use` statements that nest multi-segment paths inside a brace group.

### Why is this bad?

Grouped imports such as `use module1::blah::{module2::bleep::Type, module3::Bleh};`
or `use std::{collections::HashMap, path::PathBuf};` hide the modules being imported
inside the group and make the import list hard to scan. Prefer one flat `use`
statement per leaf path, reserving brace grouping for sibling leaf items such as
`use std::collections::{HashMap, HashSet};`.

### Example

```rust,ignore
use module1::blah::{module2::bleep::Type, module3::Bleh};
use std::{collections::HashMap, path::PathBuf};
```

Use instead:

```rust,ignore
use module1::blah::module2::bleep::Type;
use module1::blah::module3::Bleh;

use std::collections::HashMap;
use std::path::PathBuf;
```

Sibling leaf grouping is still permitted:

```rust,ignore
use std::collections::{HashMap, HashSet};
```
