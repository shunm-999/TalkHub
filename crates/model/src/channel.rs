use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
struct ChannelId(pub String);

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
struct Channel {
    id: ChannelId,
    name: String,
    description: String,
}
