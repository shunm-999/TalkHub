use actix_web::dev::Response;
use actix_web::http::StatusCode;
use actix_web::Responder;
use actix_web::web::Data;

use crate::context::TalkHubContext;

pub(crate) async fn create_channel(context: Data<TalkHubContext>) -> impl Responder {
    Response::new(StatusCode::CREATED)
}
