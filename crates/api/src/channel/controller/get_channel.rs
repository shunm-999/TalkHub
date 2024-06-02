use actix_web::dev::Response;
use actix_web::http::StatusCode;
use actix_web::web::{Data, Path};
use actix_web::{web, Responder};

use crate::context::TalkHubContext;

pub(crate) async fn get_channel(
    context: Data<TalkHubContext>,
    path: Path<(String)>,
) -> impl Responder {
    Response::new(StatusCode::OK)
}
