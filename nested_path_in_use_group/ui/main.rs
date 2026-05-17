#![allow(unused_imports, dead_code)]

mod outer {
    pub mod alpha {
        pub mod deep {
            pub struct X;
        }
        pub struct A;
    }
    pub mod beta {
        pub struct Y;
        pub struct B;
    }
    pub mod gamma {
        pub struct C;
    }
    pub mod delta {
        pub struct D;
    }
}

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use std::fmt::Display;

use outer::{alpha, beta};

use outer::alpha::A as AlphaA;
use outer::alpha::deep::X;

use outer::{gamma::C, delta::D};

use outer::{alpha::deep::X as Z, beta::Y};

use outer::{alpha::{A as A2, deep::X as X2}, beta::B};

use std::{collections::BTreeMap, path::Component};

fn main() {}
