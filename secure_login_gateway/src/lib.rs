use thiserror::Error;

const MIN_PASS_LENGTH: u32 = 8;
const MAX_PASS_LENGTH: u32 = 16;

#[derive(Debug, Error)]
pub enum PassError {
    #[error("Invalid password length, should be between {MIN_PASS_LENGTH} to {MAX_PASS_LENGTH}")]
    InvalidPassLength,

    #[error("A password must contain at least one small letter")]
    NoSmallLetter,


}