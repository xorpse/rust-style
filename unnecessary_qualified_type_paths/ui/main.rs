use std::fs;
use std::io;
use std::path::PathBuf;

mod inner {
    pub struct Local;
    pub mod nested {
        pub struct Nested;
    }

    pub trait LocalTrait {}
}

use crate::inner::Local;
use crate::inner::nested;
use crate::inner::nested::Nested;

struct BadFields {
    file: std::fs::File,
    local: crate::inner::nested::Nested,
}

enum BadEnum {
    Path(std::path::PathBuf),
}

type BadAlias = std::path::PathBuf;

fn bad_arg(_: crate::inner::Local) {}

fn bad_return() -> std::path::PathBuf {
    PathBuf::new()
}

fn bad_generic(_: Option<std::path::PathBuf>) {}

fn bad_trait_bound<T: std::fmt::Debug + crate::inner::LocalTrait>(_: T) {}

fn bad_dyn(_: Box<dyn std::fmt::Debug>) {}

fn bad_impl(_: impl std::fmt::Debug) {}

fn bad_global(_: ::std::path::PathBuf) {}

fn good_imported(local: Local, nested_value: Nested) -> PathBuf {
    let _ = (local, nested_value);
    PathBuf::new()
}

fn good_module_forms(file: fs::File, result: io::Result<()>, nested_value: nested::Nested) {
    let _ = (file, result, nested_value);
}

fn good_assoc() -> <Vec<PathBuf> as IntoIterator>::Item {
    PathBuf::new()
}

fn main() {
    let _bad_local: std::path::PathBuf = PathBuf::new();
    let _good_local: io::Result<()> = Ok(());
}
