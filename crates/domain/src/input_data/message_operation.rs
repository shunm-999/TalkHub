use talk_hub_model::channel::ChannelId;
use talk_hub_model::message::MessageId;

pub struct MessageCreation {
    pub channel_id: ChannelId,
    pub content: String,
}

pub struct MessageUpdate {
    pub id: MessageId,
    pub content: String,
}

pub struct MessageDeletion {
    pub id: MessageId,
}
