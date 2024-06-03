use talk_hub_model::channel::Channel;

use crate::crates::channel_operation::GetChannels;
use crate::result::TalkHubResult;

pub struct GetChannelsUseCase {
    repository: Box<dyn GetChannels>,
}

impl GetChannelsUseCase {
    pub fn new(repository: Box<dyn GetChannels>) -> Self {
        Self { repository }
    }

    pub async fn invoke(self) -> TalkHubResult<Vec<Channel>> {
        self.repository.as_ref().get_channels().await
    }
}
