use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct ChannelId(pub String);

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Channel {
    id: ChannelId,
    name: String,
    description: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
