use async_trait::async_trait;

use talk_hub_domain::crates::channel_operation::{
    DeleteChannel, GetChannel, GetChannels, UpdateChannel,
};
use talk_hub_domain::error::TalkHubResult;
use talk_hub_domain::input_data::channel_operation::{ChannelDeletion, ChannelUpdate};
use talk_hub_model::channel::{Channel, ChannelId};

pub struct ChannelRepository {}

impl ChannelRepository {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl GetChannel for ChannelRepository {
    async fn get_channel(&self, channel_id: ChannelId) -> TalkHubResult<Channel> {
        unimplemented!()
    }
}

#[async_trait]
impl GetChannels for ChannelRepository {
    async fn get_channels(&self) -> TalkHubResult<Vec<Channel>> {
        unimplemented!()
    }
}

#[async_trait]
impl UpdateChannel for ChannelRepository {
    async fn update_channel(&self, operation: ChannelUpdate) -> TalkHubResult<Channel> {
        unimplemented!()
    }
}

#[async_trait]
impl DeleteChannel for ChannelRepository {
    async fn delete_channel(&self, operation: ChannelDeletion) -> TalkHubResult<()> {
        unimplemented!()
    }
}
