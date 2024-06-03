use async_trait::async_trait;

use talk_hub_model::channel::{Channel, ChannelId};

use crate::input_data::channel_operation::{ChannelCreation, ChannelDeletion, ChannelUpdate};
use crate::result::TalkHubResult;

#[async_trait]
pub trait GetChannel {
    async fn get_channel(&self, channel_id: ChannelId) -> TalkHubResult<Channel>;
}

#[async_trait]
pub trait GetChannels {
    async fn get_channels(&self) -> TalkHubResult<Vec<Channel>>;
}

#[async_trait]
pub trait CreateChannel {
    async fn create_channel(&self, operation: ChannelCreation) -> TalkHubResult<Channel>;
}

#[async_trait]
pub trait UpdateChannel {
    async fn update_channel(&self, operation: ChannelUpdate) -> TalkHubResult<Channel>;
}

#[async_trait]
pub trait DeleteChannel {
    async fn delete_channel(&self, operation: ChannelDeletion) -> TalkHubResult<()>;
}
