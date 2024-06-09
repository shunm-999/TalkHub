use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use uuid::Uuid;

use crate::channel::ChannelId;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct MessageId(pub String);

impl From<MessageId> for Uuid {
    fn from(value: MessageId) -> Uuid {
        Uuid::parse_str(&value.0).unwrap()
    }
}

pub struct Message {
    pub id: MessageId,
    pub content: String,
    pub channel_id: ChannelId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Display for MessageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
