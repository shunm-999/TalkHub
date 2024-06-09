use serde::{Deserialize, Serialize};
use talk_hub_domain::input_data::channel_operation::ChannelCreation;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateChannelRequest {
    pub name: String,
    pub description: String,
}

impl Into<ChannelCreation> for CreateChannelRequest {
    fn into(self) -> ChannelCreation {
        ChannelCreation {
            name: self.name,
            description: self.description,
        }
    }
}
