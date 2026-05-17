#![allow(unused_imports, dead_code)]

extern crate self as fakecrate;

use fakecrate::LocalThing as L1;
use std::collections::HashMap;
use crate::OtherThing as O1;

use std::path::PathBuf;
use fakecrate::OtherThing as Renamed;
use std::fs::File;

pub struct LocalThing;
pub struct OtherThing;

mod inner {
    use fakecrate::LocalThing as FakeInInner;
    use std::io;
    use crate::inner::InnerThing as Self0;

    pub struct InnerThing;
}

mod good {
    use std::collections::BTreeMap;
    use std::path::Path;

    use fakecrate::LocalThing as Good1;

    use crate::OtherThing as Good2;
}

mod good_single_group {
    use std::collections::HashSet;
    use std::path::Component;
}

mod good_with_blank_lines {
    use std::collections::VecDeque;

    use fakecrate::LocalThing as Good3;
}

fn main() {}
