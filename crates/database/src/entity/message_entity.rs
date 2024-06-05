use chrono::{DateTime, Utc};
use diesel::{Identifiable, Queryable, Selectable};

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = crate::schema::message)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MessageEntity {
    id: i32,
    content: String,
    channel_id: i32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
