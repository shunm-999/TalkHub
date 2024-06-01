use crate::channel::controller::get_channels;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/channels").route("", web::get().to(get_channels)));
}
