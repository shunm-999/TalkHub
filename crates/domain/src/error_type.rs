use std::fmt::{Display, Formatter};
use crate::error::TalkHubErrorType;

impl TalkHubErrorType for ErrorType {}

#[derive(Debug)]
pub enum ErrorType {
    Unknown(String),
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorType::Unknown(message) => write!(f, "{}", message),
        }
    }
}
