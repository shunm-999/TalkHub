use actix_web::http::StatusCode;
use actix_web::web::{Data, Json};

use talk_hub_domain::result::TalkHubResult;
use talk_hub_domain::usecase::channel::get_channels_usecase::GetChannelsUseCase;
use talk_hub_repository::channel_repository::ChannelRepository;

use crate::channel::data::response::GetChannelsResponse;
use crate::context::TalkHubContext;

pub(crate) async fn get_channels(context: Data<TalkHubContext>) -> TalkHubResult<(Json<GetChannelsResponse>, StatusCode)> {
    let usecase = GetChannelsUseCase::new(
        Box::new(ChannelRepository::new()),
    );
    let channels = usecase.invoke().await?;
    let response = GetChannelsResponse {
        channels: channels.into()
    };
    Ok((Json(response), StatusCode::OK))
}
