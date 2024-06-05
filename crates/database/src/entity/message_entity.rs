use chrono::{DateTime, Utc};
use diesel::{Identifiable, Queryable, Selectable};
use talk_hub_model::channel::ChannelId;
use talk_hub_model::message::{Message, MessageId};
use uuid::Uuid;

#[derive(Queryable, Identifiable, Selectable, Debug)]
#[diesel(table_name = crate::schema::message)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MessageEntity {
    id: Uuid,
    content: String,
    channel_id: Uuid,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Into<Message> for MessageEntity {
    fn into(self) -> Message {
        Message {
            id: MessageId(self.id.into()),
            content: self.content,
            channel_id: ChannelId(self.channel_id.into()),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
