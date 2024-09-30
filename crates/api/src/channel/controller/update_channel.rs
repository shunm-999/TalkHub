use crate::channel::data::request::{CreateChannelRequest, UpdateChannelRequest};
use crate::channel::data::response::{CreateChannelResponse, UpdateChannelResponse};
use crate::common::api_result::ApiResult;
use crate::context::TalkHubContext;
use actix_web::dev::Response;
use actix_web::http::StatusCode;
use actix_web::web::{Data, Json, Path};
use actix_web::{web, Responder};

pub(crate) async fn update_channel(
    context: Data<TalkHubContext>,
    path: Path<(String)>,
    body: Json<UpdateChannelRequest>,
) -> ApiResult<Json<UpdateChannelResponse>> {
    let pool = &mut context.pool();
    let request = body.into_inner();
}
