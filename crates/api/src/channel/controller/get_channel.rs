use actix_web::http::StatusCode;
use actix_web::web::{Data, Json, Path};

use talk_hub_domain::usecase::channel::get_channel_usecase::GetChannelUseCase;
use talk_hub_model::channel::ChannelId;
use talk_hub_repository::channel_repository::ChannelRepository;

use crate::channel::data::response::GetChannelResponse;
use crate::common::api_result::ApiResult;
use crate::context::TalkHubContext;

pub(crate) async fn get_channel(
    context: Data<TalkHubContext>,
    path: Path<(String)>,
) -> ApiResult<Json<GetChannelResponse>> {
    let pool = &mut context.pool();
    let usecase = GetChannelUseCase::new(ChannelRepository::new(pool));

    let channel_id = ChannelId(path.into_inner());
    let channel = usecase.invoke(channel_id).await?;
    let response = GetChannelResponse {
        channel: channel.into(),
    };
    Ok((Json(response), StatusCode::OK))
}
