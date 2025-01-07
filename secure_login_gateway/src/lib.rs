pub mod error;

use error::{PassError, PassResult};

/// Limits for password length
const MIN_PASS_LENGTH: usize = 8;
const MAX_PASS_LENGTH: usize = 16;

/// This function checks validilty of a password.
/// 
/// # Example
/// ```
/// use secure_login_gateway::error::PassError;
/// use secure_login_gateway::validate_password;
/// 
/// assert_eq!(validate_password("1234"), Err(PassError::InvalidPassLength));
/// assert_eq!(validate_password("01234567890123456789"), Err(PassError::InvalidPassLength));
/// assert_eq!(validate_password("12345678AA"), Err(PassError::NoSmallLetter));
/// assert_eq!(validate_password("12345678aa"), Err(PassError::NoCapitalLetter));
/// assert_eq!(validate_password("aaaaaAAAAA"), Err(PassError::NoDigit));
/// assert_ne!(validate_password("njsnkfbasfbha"), Ok(()));
/// assert_eq!(validate_password("1234aaaaaAAAAA"), Ok(()));
/// ```
pub fn validate_password(password: &str) -> PassResult<()> {
    if password.len() < MIN_PASS_LENGTH || password.len() > MAX_PASS_LENGTH {
        return Err(PassError::InvalidPassLength);
    }

    if !password.chars().any(|c| c.is_lowercase()) {
        return Err(PassError::NoSmallLetter);
    }

    if !password.chars().any(|c| c.is_uppercase()) {
        return Err(PassError::NoCapitalLetter);
    }

    if !password.chars().any(|c| c.is_ascii_digit()) {
        return Err(PassError::NoDigit);
    }
    Ok(())
}