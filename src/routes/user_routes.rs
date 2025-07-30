use actix_web::web;
use crate::handlers::user_handler::{
    create_user, delete_user, get_user, get_users, update_user,
};

pub fn init_user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::get().to(get_users))          // GET /users
            .route("", web::post().to(create_user))       // POST /users
            .route("/{id}", web::get().to(get_user))      // GET /users/{id}
            .route("/{id}", web::put().to(update_user))   // PUT /users/{id}
            .route("/{id}", web::delete().to(delete_user)), // DELETE /users/{id}
    );
}
