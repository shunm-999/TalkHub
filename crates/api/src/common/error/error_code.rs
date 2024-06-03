use serde::{Deserialize, Serialize};

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
