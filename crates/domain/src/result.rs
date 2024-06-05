use crate::errors::error::TalkHubError;

pub type TalkHubResult<T> = Result<T, TalkHubError>;
