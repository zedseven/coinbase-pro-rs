use serde::{Deserialize, Serialize};
use std::fmt;

// Message
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Error {
    message: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
