use chrono::{DateTime, Utc};
use diesel::{Identifiable, Queryable, Selectable};
use talk_hub_model::channel::{Channel, ChannelId};
use uuid::Uuid;

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = crate::schema::channel)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ChannelEntity {
    id: Uuid,
    name: String,
    description: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Into<Channel> for ChannelEntity {
    fn into(self) -> Channel {
        Channel {
            id: ChannelId(self.id.into()),
            name: self.name,
            description: self.description,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
