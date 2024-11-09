use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("align too large")]
    AlignTooLarge,

    #[error("size too large")]
    SizeTooLarge,
}
