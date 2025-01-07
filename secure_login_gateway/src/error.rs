//! This module defines password error result type.
use thiserror::Error;

/// Password possible errors
#[derive(Debug, Error, PartialEq, Eq)]
pub enum PassError {
    #[error("Invalid password length")]
    InvalidPassLength,

    #[error("A password must contain at least one small letter")]
    NoSmallLetter,

    #[error("A password must contain at least one capital letter")]
    NoCapitalLetter,

    #[error("A password must contain at least one digit")]
    NoDigit,
}

pub type PassResult<T> = Result<T, PassError>;