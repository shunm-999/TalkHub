use std::fmt::{Debug, Display};

use tracing_error::SpanTrace;

use crate::errors::error_type::TalkHubErrorType;
use crate::result::TalkHubResult;

pub struct TalkHubError {
    pub error_type: TalkHubErrorType,
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
            error_type: TalkHubErrorType::Unknown(format!("{}", &cause)),
            inner: cause,
            context: SpanTrace::capture(),
        }
    }
}

impl Debug for TalkHubError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TalkHubError")
            .field("message", &self.error_type)
            .field("inner", &self.inner)
            .field("context", &self.context)
            .finish()
    }
}

impl Display for TalkHubError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: ", self.error_type)?;
        writeln!(f, "{}", self.inner)?;
        std::fmt::Display::fmt(&self.context, f)
    }
}

impl From<TalkHubErrorType> for TalkHubError {
    fn from(value: TalkHubErrorType) -> Self {
        let inner = anyhow::anyhow!("{}", value);
        TalkHubError {
            error_type: value,
            inner,
            context: SpanTrace::capture(),
        }
    }
}

pub trait IntoTalkHubError<T, E: Into<anyhow::Error>> {
    fn with_error_type(self, error_type: TalkHubErrorType) -> TalkHubResult<T>;

    fn map_error_type(self, op: impl FnOnce(&E) -> TalkHubErrorType) -> TalkHubResult<T>;
}

impl<T, E: Into<anyhow::Error>> IntoTalkHubError<T, E> for Result<T, E> {
    fn with_error_type(self, error_type: TalkHubErrorType) -> TalkHubResult<T> {
        self.map_err(|e| TalkHubError {
            error_type,
            inner: e.into(),
            context: SpanTrace::capture(),
        })
    }

    fn map_error_type(self, op: impl FnOnce(&E) -> TalkHubErrorType) -> TalkHubResult<T> {
        self.map_err(|e| TalkHubError {
            error_type: op(&e),
            inner: e.into(),
            context: SpanTrace::capture(),
        })
    }
}
