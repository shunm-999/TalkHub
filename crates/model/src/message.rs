use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::channel::ChannelId;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct MessageId(pub String);

pub struct Message {
    id: MessageId,
    content: String,
    channel_id: ChannelId,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Display for MessageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
