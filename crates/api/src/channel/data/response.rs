use crate::common::view_model::channel_view::ChannelViewModel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetChannelsResponse {
    pub channels: Vec<ChannelViewModel>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetChannelResponse {
    pub channel: ChannelViewModel,
}
