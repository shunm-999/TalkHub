use std::fmt::{Display, Formatter};

use talk_hub_model::channel::ChannelId;
use talk_hub_model::message::MessageId;

use crate::errors::channel::channel_error_context::ChannelErrorContext;
use crate::errors::message::message_error_context::MessageErrorContext;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TalkHubErrorType {
    Unknown(String),
    ChannelError(ChannelErrorContext),
    MessageError(MessageErrorContext),
    UnAuthorized,
}

impl Display for TalkHubErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TalkHubErrorType::Unknown(_) => {
                write!(f, "Unknown error")
            }
            TalkHubErrorType::ChannelError(context) => {
                write!(f, "{}", context)
            }
            TalkHubErrorType::MessageError(context) => {
                write!(f, "{}", context)
            }
            TalkHubErrorType::UnAuthorized => {
                write!(f, "UnAuthorized User")
            }
        }
    }
}
