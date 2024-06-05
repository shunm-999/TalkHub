use std::fmt::Display;
use talk_hub_model::channel::ChannelId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChannelErrorContext {
    NotFound(ChannelId),
    AlreadyExists(ChannelId),
    AccessDenied(ChannelId),
    InvalidOperation(ChannelId),
}

impl Display for ChannelErrorContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChannelErrorContext::NotFound(channel_id) => {
                write!(f, "ChannelId {} not found", channel_id)
            }
            ChannelErrorContext::AlreadyExists(channel_id) => {
                write!(f, "ChannelId {} already exists", channel_id)
            }
            ChannelErrorContext::AccessDenied(channel_id) => {
                write!(f, "Access Denied to ChannelId {}", channel_id)
            }
            ChannelErrorContext::InvalidOperation(channel_id) => {
                write!(f, "Invalid operation for ChannelId {}", channel_id)
            }
        }
    }
}
