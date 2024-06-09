use async_trait::async_trait;
use std::ops::DerefMut;
use talk_hub_database::dao::channel_dao::ChannelDao;

use talk_hub_database::utils::{get_conn, DbPool};
use talk_hub_domain::crates::channel_operation::{
    DeleteChannel, GetChannel, GetChannels, UpdateChannel,
};
use talk_hub_domain::input_data::channel_operation::{ChannelDeletion, ChannelUpdate};
use talk_hub_domain::result::TalkHubResult;
use talk_hub_model::channel::{Channel, ChannelId};

pub struct ChannelRepository<'a> {
    db_pool: &'a mut DbPool<'a>,
}

impl<'a> ChannelRepository<'a> {
    pub fn new(db_pool: &'a mut DbPool<'a>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl GetChannel for ChannelRepository<'_> {
    async fn get_channel(self, channel_id: ChannelId) -> TalkHubResult<Channel> {
        unimplemented!()
    }
}

#[async_trait]
impl GetChannels for ChannelRepository<'_> {
    async fn get_channels(self) -> TalkHubResult<Vec<Channel>> {
        let mut conn = get_conn(self.db_pool).await?;
        ChannelDao::select_all(conn.deref_mut())
            .await
            .map(|channels| {
                channels
                    .into_iter()
                    .map(|channel_entity| channel_entity.into())
                    .collect()
            })
    }
}

#[async_trait]
impl UpdateChannel for ChannelRepository<'_> {
    async fn update_channel(self, operation: ChannelUpdate) -> TalkHubResult<Channel> {
        unimplemented!()
    }
}

#[async_trait]
impl DeleteChannel for ChannelRepository<'_> {
    async fn delete_channel(self, operation: ChannelDeletion) -> TalkHubResult<()> {
        unimplemented!()
    }
}
