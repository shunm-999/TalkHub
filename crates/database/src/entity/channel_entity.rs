use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Identifiable, Insertable, QueryId, Queryable, Selectable};
use uuid::Uuid;

use talk_hub_model::channel::{Channel, ChannelId};

#[derive(
    Debug, PartialEq, QueryId, Queryable, Identifiable, Selectable, Insertable, AsChangeset,
)]
#[diesel(table_name = crate::schema::channel)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ChannelEntity {
    id: Uuid,
    name: String,
    description: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl ChannelEntity {
    pub fn new(name: String, description: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            created_at: now,
            updated_at: now,
        }
    }
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
