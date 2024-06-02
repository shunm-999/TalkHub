use std::fmt::{Display, Formatter};

use actix_web::http::StatusCode;

use talk_hub_domain::error::{TalkHubError, TalkHubErrorType};
use talk_hub_domain::error_type::ErrorType;

#[derive(Debug)]
pub struct ErrorResponse {
    pub inner: TalkHubError,
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

// impl actix_web::ResponseError for ErrorResponse {
//     fn status_code(&self) -> StatusCode {
//         self.inner.error_type.
//     }
// }

fn into_status_code(error_type: &dyn TalkHubErrorType) -> StatusCode {
    match error_type {
        ErrorType::Unknown(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}