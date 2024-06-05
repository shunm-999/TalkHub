diesel::table! {
    channel(id) {
        id -> Uuid,
        name -> Text,
        description -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    message(id) {
        id -> Uuid,
        content -> Text,
        channel_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(message -> channel (channel_id));

diesel::allow_tables_to_appear_in_same_query!(channel, message);
