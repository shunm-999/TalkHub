use actix_web::http::StatusCode;

use talk_hub_domain::errors::channel::channel_error_context::ChannelErrorContext;
use talk_hub_domain::errors::error_type::TalkHubErrorType;
use talk_hub_domain::errors::message::message_error_context::MessageErrorContext;

pub struct TalkHubErrorTypeContext {
    inner: TalkHubErrorType,
}

impl From<&TalkHubErrorType> for TalkHubErrorTypeContext {
    fn from(value: &TalkHubErrorType) -> Self {
        TalkHubErrorTypeContext {
            inner: value.clone(),
        }
    }
}

impl From<&TalkHubErrorTypeContext> for StatusCode {
    fn from(&TalkHubErrorTypeContext { ref inner }: &TalkHubErrorTypeContext) -> Self {
        match inner {
            TalkHubErrorType::Unknown(_) => StatusCode::INTERNAL_SERVER_ERROR,
            TalkHubErrorType::ChannelError(context) => match context {
                ChannelErrorContext::NotFound(_) => StatusCode::NOT_FOUND,
                ChannelErrorContext::AlreadyExists(_) => StatusCode::BAD_REQUEST,
                ChannelErrorContext::AccessDenied(_) => StatusCode::FORBIDDEN,
                ChannelErrorContext::InvalidOperation(_) => StatusCode::BAD_REQUEST,
            },
            TalkHubErrorType::MessageError(context) => match context {
                MessageErrorContext::NotFound(_) => StatusCode::NOT_FOUND,
                MessageErrorContext::AccessDenied(_) => StatusCode::FORBIDDEN,
                MessageErrorContext::InvalidOperation(_) => StatusCode::BAD_REQUEST,
            },
            TalkHubErrorType::UnAuthorized => StatusCode::UNAUTHORIZED,
        }
    }
}
