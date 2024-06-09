use talk_hub_model::channel::Channel;

use crate::crates::channel_operation::UpdateChannel;
use crate::input_data::channel_operation::ChannelUpdate;
use crate::result::TalkHubResult;

pub struct UpdateChannelUseCase<T: UpdateChannel + Sized> {
    repository: T,
}

impl<T: UpdateChannel + Sized> UpdateChannelUseCase<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn invoke(self, operation: ChannelUpdate) -> TalkHubResult<Channel> {
        self.repository.update_channel(operation).await
    }
}
