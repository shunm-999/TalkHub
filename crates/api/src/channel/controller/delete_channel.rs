use crate::channel::data::response::CreateChannelResponse;
use crate::common::api_result::ApiResult;
use crate::context::TalkHubContext;
use actix_web::body::BoxBody;
use actix_web::dev::Response;
use actix_web::http::StatusCode;
use actix_web::web::{Data, Json, Path};
use talk_hub_domain::usecase::channel::delete_channel_usecase::DeleteChannelUseCase;
use talk_hub_model::channel::ChannelId;
use talk_hub_repository::channel_repository::ChannelRepository;

pub(crate) async fn delete_channel(
    context: Data<TalkHubContext>,
    path: Path<(String)>,
) -> ApiResult<Response<BoxBody>> {
    let pool = &mut context.pool();
    let usecase = DeleteChannelUseCase::new(ChannelRepository::new(pool));

    let channel_id = ChannelId(path.into_inner());
    usecase.invoke(channel_id).await?;

    Ok((
        Response::new(StatusCode::NO_CONTENT),
        StatusCode::NO_CONTENT,
    ))
}
