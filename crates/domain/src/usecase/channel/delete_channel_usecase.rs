use crate::crates::channel_operation::DeleteChannel;
use crate::input_data::channel_operation::ChannelDeletion;
use crate::result::TalkHubResult;

pub struct DeleteChannelUseCase<T: DeleteChannel + Sized> {
    repository: T,
}

impl<T: DeleteChannel + Sized> DeleteChannelUseCase<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn invoke(self, operation: ChannelDeletion) -> TalkHubResult<()> {
        self.repository.delete_channel(operation).await
    }
}
