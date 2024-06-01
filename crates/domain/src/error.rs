use std::fmt::{Debug, Display, Formatter};

use tracing_error::SpanTrace;

pub type TalkHubResult<T> = Result<T, TalkHubError>;

pub trait TalkHubErrorType: Debug + Display {}

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

impl TalkHubErrorType for ErrorType {}

pub struct TalkHubError {
    pub error_type: Box<dyn TalkHubErrorType>,
    pub inner: anyhow::Error,
    pub context: SpanTrace,
}

impl<T> From<T> for TalkHubError
where
    T: Into<anyhow::Error>,
{
    fn from(value: T) -> Self {
        let cause = value.into();
        TalkHubError {
            error_type: Box::new(ErrorType::Unknown(format!("{}", &cause))),
            inner: cause,
            context: SpanTrace::capture(),
        }
    }
}

impl Debug for TalkHubError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TalkHubError")
            .field("message", &self.error_type.as_ref())
            .field("inner", &self.inner)
            .field("context", &self.context)
            .finish()
    }
}

impl From<Box<dyn TalkHubErrorType>> for TalkHubError {
    fn from(value: Box<dyn TalkHubErrorType>) -> Self {
        let inner = anyhow::anyhow!("{}", value.as_ref());
        TalkHubError {
            error_type: value,
            inner,
            context: SpanTrace::capture(),
        }
    }
}
