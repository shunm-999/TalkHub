use std::fmt::Display;

use talk_hub_model::message::MessageId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageErrorContext {
    NotFound(MessageId),
    AccessDenied(MessageId),
    InvalidOperation(MessageId),
}

impl Display for MessageErrorContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageErrorContext::NotFound(message_id) => {
                write!(f, "MessageId {} not found", message_id)
            }
            MessageErrorContext::AccessDenied(message_id) => {
                write!(f, "Access Denied to MessageId {}", message_id)
            }
            MessageErrorContext::InvalidOperation(message_id) => {
                write!(f, "Invalid operation for MessageId {}", message_id)
            }
        }
    }
}
