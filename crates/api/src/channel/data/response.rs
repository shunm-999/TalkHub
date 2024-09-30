use crate::common::view_model::channel_view::{ChannelViewModel, ChannelsViewModel};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetChannelsResponse {
    pub channels: ChannelsViewModel,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetChannelResponse {
    pub channel: ChannelViewModel,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateChannelResponse {
    pub channel: ChannelViewModel,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateChannelResponse {
    pub channel: ChannelViewModel,
}
