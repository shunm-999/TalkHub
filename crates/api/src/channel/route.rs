use actix_web::web;

use crate::channel::controller::create_channel::create_channel;
use crate::channel::controller::delete_channel::delete_channel;
use crate::channel::controller::get_channel::get_channel;
use crate::channel::controller::get_channels::get_channels;
use crate::channel::controller::update_channel::update_channel;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/channels")
            .route("", web::get().to(get_channels))
            .route("", web::post().to(create_channel))
            .route("/{id}", web::get().to(get_channel))
            .route("/{id}", web::put().to(update_channel))
            .route("/{id}", web::delete().to(delete_channel)),
    );
}
