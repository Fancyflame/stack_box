use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("align too large, expect maximum {expect} but requires {require}")]
    AlignTooLarge { expect: usize, require: usize },

    #[error("size too large, expect maximum {expect} but requires {require}")]
    SizeTooLarge { expect: usize, require: usize },
}
