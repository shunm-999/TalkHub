use actix_web::dev::Response;
use actix_web::http::StatusCode;
use actix_web::web::{Data, Json};

use talk_hub_domain::usecase::channel::create_channel_usecase::CreateChannelUseCase;
use talk_hub_repository::channel_repository::ChannelRepository;

use crate::channel::data::request::CreateChannelRequest;
use crate::channel::data::response::CreateChannelResponse;
use crate::common::api_result::ApiResult;
use crate::context::TalkHubContext;

pub(crate) async fn create_channel(
    context: Data<TalkHubContext>,
    body: Json<CreateChannelRequest>,
) -> ApiResult<Json<CreateChannelResponse>> {
    let pool = &mut context.pool();
    let request = body.into_inner();

    let usecase = CreateChannelUseCase::new(ChannelRepository::new(pool));
    let channel = usecase.invoke(request.into()).await?;
    let response = CreateChannelResponse {
        channel: channel.into(),
    };

    Ok((Json(response), StatusCode::CREATED))
}
