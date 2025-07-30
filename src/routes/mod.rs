// src/routes/mod.rs

use actix_web::web;

mod user_routes;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.configure(user_routes::init_user_routes);
}
