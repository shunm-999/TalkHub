use talk_hub_model::channel::{Channel, ChannelId};

use crate::crates::channel_operation::GetChannel;
use crate::result::TalkHubResult;

pub struct GetChannelUseCase<T: GetChannel + Sized> {
    repository: T,
}

impl<T: GetChannel + Sized> GetChannelUseCase<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn invoke(self, channel_id: ChannelId) -> TalkHubResult<Channel> {
        self.repository.get_channel(channel_id).await
    }
}
