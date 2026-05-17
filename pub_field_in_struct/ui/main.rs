#![allow(dead_code)]

mod inner {
    pub struct BadNamed {
        pub timeout: u64,
        pub label: String,
        private: u32,
    }

    pub struct BadMixed {
        pub a: u32,
        pub(crate) b: u32,
        pub(super) c: u32,
        d: u32,
    }

    pub struct BadTuple(pub u32, u32, pub String);

    pub struct GoodTuple(u32, String);

    pub struct GoodNamed {
        timeout: u64,
        label: String,
    }

    pub union BadUnion {
        pub a: u32,
        b: f32,
    }

    pub union GoodUnion {
        a: u32,
        b: f32,
    }

    pub enum NotLinted {
        Variant { field: u32 },
        TupleVariant(u32),
    }

    #[allow(pub_field_in_struct)]
    pub struct OptedOut {
        pub raw: *mut u8,
    }
}

fn main() {}
