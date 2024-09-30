use serde::{Deserialize, Serialize};
use talk_hub_domain::input_data::channel_operation::{ChannelCreation, ChannelUpdating};
use talk_hub_model::channel::ChannelId;
use talk_hub_util::IntoWithParam;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateChannelRequest {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateChannelRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

impl Into<ChannelCreation> for CreateChannelRequest {
    fn into(self) -> ChannelCreation {
        ChannelCreation {
            name: self.name,
            description: self.description,
        }
    }
}

impl IntoWithParam<ChannelUpdating, ChannelId> for UpdateChannelRequest {
    fn into_with_param(self, param: ChannelId) -> ChannelUpdating {
        ChannelUpdating {
            id: param,
            name: self.name,
            description: self.description,
        }
    }
}
