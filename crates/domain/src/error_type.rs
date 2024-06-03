use std::fmt::{Display, Formatter};

use talk_hub_model::channel::ChannelId;
use talk_hub_model::message::MessageId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TalkHubErrorType {
    Unknown(String),
    NotFoundChannel(ChannelId),
    NotFoundMessage(MessageId),
    UnAuthorized,
    AccessDeniedChannel(ChannelId),
    AccessDeniedMessage(MessageId),
}

impl Display for TalkHubErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TalkHubErrorType::Unknown(_) => {
                write!(f, "Unknown error")
            }
            TalkHubErrorType::NotFoundChannel(channelId) => {
                write!(f, "ChannelId {} not found", channelId)
            }
            TalkHubErrorType::NotFoundMessage(messagedId) => {
                write!(f, "MessageId {} not found", messagedId)
            }
            TalkHubErrorType::UnAuthorized => {
                write!(f, "UnAuthorized User")
            }
            TalkHubErrorType::AccessDeniedChannel(channelId) => {
                write!(f, "Access Denied to ChannelId {}", channelId)
            }
            TalkHubErrorType::AccessDeniedMessage(messageId) => {
                write!(f, "Access Denied to MessageId {}", messageId)
            }
        }
    }
}
