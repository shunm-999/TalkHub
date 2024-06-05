use serde::{Deserialize, Serialize};

use talk_hub_domain::errors::channel::channel_error_context::ChannelErrorContext;
use talk_hub_domain::errors::error_type::TalkHubErrorType;
use talk_hub_domain::errors::message::message_error_context::MessageErrorContext;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ErrorCode(pub u16);
macro_rules! error_codes {
     (
        $(
            $(#[$docs:meta])*
            ($num:expr, $konst:ident, $phrase:expr);
        )+
    ) => {
        impl ErrorCode {
            $(
                $(#[$docs])*
                pub const $konst: ErrorCode = ErrorCode($num);
            )+

            pub fn type_name(&self) -> &str {
                match self.0 {
                    $(
                        $num => $phrase,
                    )+
                    _ => "Unknown error",
                }
            }
        }
    }
}

error_codes! {
    (1, BAD_REQUEST, "Bad Request");
    (2, UNAUTHORIZED_TOKEN, "Unauthorized Token");
    (3, FORBIDDEN, "Forbidden");
    (4, NOT_FOUND, "Not Found");
    (5, INTERNAL_SERVER_ERROR, "Internal Server Error");
    (6, SERVICE_MAINTENANCE, "Service Maintenance");
}

impl Into<u16> for ErrorCode {
    fn into(self) -> u16 {
        self.0
    }
}

impl From<&TalkHubErrorType> for ErrorCode {
    fn from(value: &TalkHubErrorType) -> Self {
        match value {
            TalkHubErrorType::Unknown(_) => ErrorCode::INTERNAL_SERVER_ERROR,
            TalkHubErrorType::ChannelError(error_context) => error_context.into(),
            TalkHubErrorType::MessageError(error_context) => error_context.into(),
            TalkHubErrorType::UnAuthorized => ErrorCode::UNAUTHORIZED_TOKEN,
        }
    }
}

impl From<&ChannelErrorContext> for ErrorCode {
    fn from(context: &ChannelErrorContext) -> Self {
        match context {
            ChannelErrorContext::NotFound(_) => ErrorCode::NOT_FOUND,
            ChannelErrorContext::AlreadyExists(_) => ErrorCode::BAD_REQUEST,
            ChannelErrorContext::AccessDenied(_) => ErrorCode::FORBIDDEN,
            ChannelErrorContext::InvalidOperation(_) => ErrorCode::BAD_REQUEST,
        }
    }
}

impl From<&MessageErrorContext> for ErrorCode {
    fn from(context: &MessageErrorContext) -> Self {
        match context {
            MessageErrorContext::NotFound(_) => ErrorCode::NOT_FOUND,
            MessageErrorContext::AccessDenied(_) => ErrorCode::FORBIDDEN,
            MessageErrorContext::InvalidOperation(_) => ErrorCode::BAD_REQUEST,
        }
    }
}
