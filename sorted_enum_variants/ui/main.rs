#![allow(dead_code, non_camel_case_types)]

enum BadOrder {
    Charlie,
    Alpha,
    Bravo,
}

enum CaseInsensitive {
    apple,
    Banana,
    cherry,
}

enum WrongCase {
    Zebra,
    apple,
}

enum GoodOrder {
    Alpha,
    Bravo,
    Charlie,
    Delta,
}

enum SingleVariant {
    Solo,
}

enum Empty {}

enum WithDiscriminants {
    Charlie = 3,
    Alpha = 1,
    Bravo = 2,
}

#[allow(sorted_enum_variants)]
enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

enum WithFields {
    Beta { x: u32 },
    Alpha(u32),
    Charlie,
}

fn main() {}
