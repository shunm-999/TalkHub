use talk_hub_model::channel::Channel;

use crate::crates::channel_operation::GetChannels;
use crate::result::TalkHubResult;

pub struct GetChannelsUseCase<T: GetChannels + Sized> {
    repository: T,
}

impl<T: GetChannels + Sized> GetChannelsUseCase<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn invoke(self) -> TalkHubResult<Vec<Channel>> {
        self.repository.get_channels().await
    }
}
