use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ApiError {
    message: String,
}

impl ApiError {
    pub fn new(message: &str) -> ApiError {
        ApiError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ApiError {}
