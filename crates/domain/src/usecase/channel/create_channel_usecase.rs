use talk_hub_model::channel::Channel;

use crate::crates::channel_operation::CreateChannel;
use crate::input_data::channel_operation::ChannelCreation;
use crate::result::TalkHubResult;

pub struct CreateChannelUseCase<T: CreateChannel + Sized> {
    repository: T,
}

impl<T: CreateChannel + Sized> CreateChannelUseCase<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn invoke(self, operation: ChannelCreation) -> TalkHubResult<Channel> {
        self.repository.create_channel(operation).await
    }
}
