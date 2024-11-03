use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Define custom error type
#[derive(Debug, candid::CandidType, Deserialize, Serialize)]
pub enum CustomError {
    MissingField(&'static str),
    custom(String),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::MissingField(field) => write!(f, "Missing required field: {}", field),
            CustomError::custom(err) => Ok(ic_cdk::print(err)),
        }
    }
}
impl Error for CustomError {}
