use diesel::{ExpressionMethods, Identifiable, QueryDsl, SelectableHelper};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use talk_hub_domain::errors::channel::channel_error_context::ChannelErrorContext;
use talk_hub_domain::errors::error::IntoTalkHubError;
use talk_hub_domain::errors::error_type::TalkHubErrorType;
use talk_hub_domain::result::TalkHubResult;
use talk_hub_model::channel::ChannelId;

use crate::entity::channel_entity::ChannelEntity;
use crate::error::diesel_error::DieselError;
use crate::schema::channel;

pub struct ChannelDao {}

impl ChannelDao {
    pub async fn select_all(conn: &mut AsyncPgConnection) -> TalkHubResult<Vec<ChannelEntity>> {
        let result = channel::table
            .select(ChannelEntity::as_select())
            .load(conn)
            .await;
        result.map_error_type(|e| convert_to_error_type(e, ChannelDaoOperation::SelectAll))
    }

    pub async fn select_by_id(
        conn: &mut AsyncPgConnection,
        channel_id: Uuid,
    ) -> TalkHubResult<ChannelEntity> {
        let result = channel::table.find(&channel_id).first(conn).await;
        result.map_error_type(|e| {
            convert_to_error_type(
                e,
                ChannelDaoOperation::SelectById(ChannelId(channel_id.to_string())),
            )
        })
    }

    pub async fn insert(
        conn: &mut AsyncPgConnection,
        channel_entity: ChannelEntity,
    ) -> TalkHubResult<ChannelEntity> {
        let result = diesel::insert_into(channel::table)
            .values(&channel_entity)
            .returning(ChannelEntity::as_returning())
            .get_result(conn)
            .await;
        result.map_error_type(|e| {
            convert_to_error_type(
                e,
                ChannelDaoOperation::Insert(ChannelId(channel_entity.id().to_string())),
            )
        })
    }

    pub async fn update(
        conn: &mut AsyncPgConnection,
        channel_id: Uuid,
        name: Option<String>,
        description: Option<String>,
    ) -> TalkHubResult<ChannelEntity> {
        let name = name.map(|name| channel::name.eq(name));
        let description = description.map(|description| channel::description.eq(description));

        let result = diesel::update(channel::table.filter(channel::id.eq(&channel_id)))
            .set((name, description))
            .get_result(conn)
            .await;
        result.map_error_type(|e| {
            convert_to_error_type(
                e,
                ChannelDaoOperation::Update(ChannelId(channel_id.to_string())),
            )
        })
    }

    pub async fn delete(
        conn: &mut AsyncPgConnection,
        channel_id: Uuid,
    ) -> TalkHubResult<ChannelEntity> {
        let result = diesel::delete(channel::table.find(&channel_id))
            .returning(ChannelEntity::as_returning())
            .get_result(conn)
            .await;
        result.map_error_type(|e| {
            convert_to_error_type(
                e,
                ChannelDaoOperation::Delete(ChannelId(channel_id.to_string())),
            )
        })
    }
}

enum ChannelDaoOperation {
    SelectAll,
    SelectById(ChannelId),
    Insert(ChannelId),
    Update(ChannelId),
    Delete(ChannelId),
}

fn convert_to_error_type(e: &DieselError, operation: ChannelDaoOperation) -> TalkHubErrorType {
    match e {
        DieselError::NotFound => match operation {
            ChannelDaoOperation::SelectById(channel_id) => {
                TalkHubErrorType::ChannelError(ChannelErrorContext::NotFound(channel_id))
            }
            _ => TalkHubErrorType::Unknown("Unknown error".to_string()),
        },
        _ => TalkHubErrorType::Unknown("Unknown error".to_string()),
    }
}
