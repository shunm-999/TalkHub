diesel::table! {
    channel(id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    message(id) {
        id -> Int4,
        content -> Text,
        channel_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(message -> channel (channel_id));

diesel::allow_tables_to_appear_in_same_query!(channel, message);
