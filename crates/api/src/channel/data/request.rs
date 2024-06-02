use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetChannelRequest {
    pub channel_id: i32,
}
