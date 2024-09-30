use serde::{Deserialize, Serialize};

use talk_hub_model::channel::Channel;

use crate::common::view_model::datetime_view::DateTimeViewModel;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChannelViewModel {
    id: String,
    name: String,
    description: String,
    #[serde(rename = "createdAt")]
    created_at: DateTimeViewModel,
    #[serde(rename = "updatedAt")]
    updated_at: DateTimeViewModel,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChannelsViewModel(Vec<ChannelViewModel>);

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

impl<T: IntoIterator<Item = Channel>> From<T> for ChannelsViewModel {
    fn from(value: T) -> Self {
        ChannelsViewModel(value.into_iter().map(ChannelViewModel::from).collect())
    }
}
