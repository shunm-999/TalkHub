use crate::channel::data::request::UpdateChannelRequest;
use crate::channel::data::response::UpdateChannelResponse;
use crate::common::api_result::ApiResult;
use crate::context::TalkHubContext;
use actix_web::http::StatusCode;
use actix_web::web::{Data, Json, Path};
use talk_hub_domain::usecase::channel::update_channel_usecase::UpdateChannelUseCase;
use talk_hub_model::channel::ChannelId;
use talk_hub_repository::channel_repository::ChannelRepository;
use talk_hub_util::IntoWithParam;

pub(crate) async fn update_channel(
    context: Data<TalkHubContext>,
    path: Path<(String)>,
    body: Json<UpdateChannelRequest>,
) -> ApiResult<Json<UpdateChannelResponse>> {
    let pool = &mut context.pool();
    let request = body.into_inner();

    let usecase = UpdateChannelUseCase::new(ChannelRepository::new(pool));
    let updating = request.into_with_param(ChannelId(path.into_inner()));
    let channel = usecase.invoke(updating).await?;
    let response = UpdateChannelResponse {
        channel: channel.into(),
    };
    Ok((Json(response), StatusCode::OK))
}
