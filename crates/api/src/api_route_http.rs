use actix_web::web;

use crate::channel;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("api/v1").configure(|cfg| {
        channel::route::config(cfg);
    }));
}
