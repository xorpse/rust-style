#![allow(dead_code)]

struct BadStruct {
    a: u32,

    b: u32,
    c: u32,


    d: u32,
}

struct GoodStruct {
    a: u32,
    b: u32,
    c: u32,
}

struct WithComments {
    a: u32,
    // an explanatory comment
    b: u32,
    /// a doc comment
    c: u32,
}

struct InlineTuple(u32, u32, u32);

struct MultiLineTuple(
    u32,

    u32,
);

union BadUnion {
    a: u32,

    b: f32,
}

union GoodUnion {
    a: u32,
    b: f32,
}

enum BadEnum {
    Alpha,

    Bravo,

    Charlie { x: u32, y: u32 },
    Delta {
        x: u32,

        y: u32,
    },
}

enum GoodEnum {
    Alpha,
    Bravo,
    Charlie { x: u32, y: u32 },
    Delta {
        x: u32,
        y: u32,
    },
}

enum WithAttrsAndDocs {
    /// alpha doc
    Alpha,
    #[allow(dead_code)]
    Bravo,
    /// charlie doc
    Charlie,
}

fn main() {}
