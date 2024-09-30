use crate::crates::channel_operation::DeleteChannel;
use crate::result::TalkHubResult;
use talk_hub_model::channel::{Channel, ChannelId};

pub struct DeleteChannelUseCase<T: DeleteChannel + Sized> {
    repository: T,
}

impl<T: DeleteChannel + Sized> DeleteChannelUseCase<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn invoke(self, channel_id: ChannelId) -> TalkHubResult<Channel> {
        self.repository.delete_channel(channel_id).await
    }
}
