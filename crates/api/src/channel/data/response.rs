use crate::common::view_model::channel_view::{ChannelsViewModel, ChannelViewModel};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetChannelsResponse {
    pub channels: ChannelsViewModel,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetChannelResponse {
    pub channel: ChannelViewModel,
}
