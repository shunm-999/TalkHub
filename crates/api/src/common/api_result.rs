use crate::common::error::error_response::ErrorResponse;
use actix_web::http::StatusCode;
use actix_web::{Error, Responder};

pub type ApiResult<T>
where
    T: Responder,
= Result<(T, StatusCode), ErrorResponse>;
