use std::fmt::{Display, Formatter};

use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

use talk_hub_domain::errors::error::TalkHubError;

use crate::common::error::error_code::ErrorCode;
use crate::common::error::status_code::TalkHubErrorTypeContext;

#[derive(Debug)]
pub struct ErrorResponse {
    pub inner: TalkHubError,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
struct ErrorResponseContext {
    code: u16,
    #[serde(rename = "type")]
    type_str: String,
    message: String,
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl From<TalkHubError> for ErrorResponse {
    fn from(value: TalkHubError) -> Self {
        ErrorResponse { inner: value }
    }
}

impl From<&ErrorResponse> for ErrorResponseContext {
    fn from(value: &ErrorResponse) -> Self {
        let code: ErrorCode = (&value.inner.error_type).into();
        let type_name = code.type_name();
        ErrorResponseContext {
            code: code.0,
            type_str: type_name.to_string(),
            message: format!("{}", value.inner.error_type),
        }
    }
}

impl actix_web::ResponseError for ErrorResponse {
    fn status_code(&self) -> StatusCode {
        let error_type = &self.inner.error_type;
        (&TalkHubErrorTypeContext::from(error_type)).into()
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let error_context: ErrorResponseContext = self.into();
        HttpResponse::build(self.status_code()).json(error_context)
    }
}
