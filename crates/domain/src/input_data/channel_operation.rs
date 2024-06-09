use talk_hub_model::channel::ChannelId;

pub struct ChannelCreation {
    pub name: String,
    pub description: String,
}

pub struct ChannelUpdate {
    pub id: ChannelId,
    pub name: Option<String>,
    pub description: Option<String>,
}

pub struct ChannelDeletion {
    pub id: ChannelId,
}
