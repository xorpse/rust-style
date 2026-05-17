#![allow(dead_code, unused_imports)]

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("a")]
    Alpha,
}

#[derive(thiserror::Error, Debug)]
pub enum FooError {
    #[error("c")]
    Charlie,
    #[error("a")]
    Alpha,
    #[error("b")]
    Bravo,
}

pub enum BareError {
    Variant,
}

#[derive(Error, Debug)]
pub enum ConstructorError {
    #[error("a")]
    NoCtor { reason: String },
    #[error("b")]
    HasCtor(u32),
    #[error("c")]
    WithFrom(#[from] std::io::Error),
    #[error("d")]
    Unit,
}

impl ConstructorError {
    pub fn has_ctor(value: u32) -> Self {
        Self::HasCtor(value)
    }
}

#[derive(Error, Debug)]
pub enum AllGoodError {
    #[error("a")]
    Apple,
    #[error("b")]
    Banana { weight: u32 },
    #[error("c")]
    Cherry,
    #[error("d")]
    Diced(#[from] std::fmt::Error),
}

impl AllGoodError {
    pub fn banana(weight: u32) -> Self {
        Self::Banana { weight }
    }
}

pub struct NotAnError {
    value: u32,
}

fn main() {}
