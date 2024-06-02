use serde::{Deserialize, Serialize};
use talk_hub_model::channel::Channel;

use crate::common::view_model::datetime_view::DateTimeViewModel;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChannelViewModel {
    id: String,
    name: String,
    description: String,
    created_at: DateTimeViewModel,
    updated_at: DateTimeViewModel,
}

impl From<Channel> for ChannelViewModel {
    fn from(channel: Channel) -> Self {
        ChannelViewModel {
            id: channel.id.to_string(),
            name: channel.name,
            description: channel.description,
            created_at: channel.created_at.into(),
            updated_at: channel.updated_at.into(),
        }
    }
}
