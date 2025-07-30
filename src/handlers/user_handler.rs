// src/handlers/user_handler.rs

use actix_web::{web, HttpResponse};
use crate::errors::ApiError;
use crate::models::user::{NewUser, UpdateUser};
use crate::services::user_service::{
    create_user_service, delete_user_service, get_user_service, get_users_service,
    update_user_service,
};
use crate::db::DbPool;

/// GET /users — получить всех пользователей
pub async fn get_users(pool: web::Data<DbPool>) -> Result<HttpResponse, ApiError> {
    let pool_clone = pool.clone();
    // web::block возвращает Result<ClosureReturn, BlockingError>
    // ClosureReturn у нас = Result<Vec<User>, ApiError>
    let result = web::block(move || get_users_service(pool_clone.get_ref())).await;

    match result {
        // Первый уровень: наш блок выполнился без Panic-а/BlockingError
        Ok(inner) => match inner {
            // Второй уровень: сервис вернул Ok(Vec<User>)
            Ok(users) => Ok(HttpResponse::Ok().json(users)),
            // Второй уровень: сервис вернул Err(ApiError)
            Err(api_err) => Err(api_err),
        },
        // Первый уровень: блокирование не удалось (BlockingError)
        Err(_) => Err(ApiError::InternalError),
    }
}

/// GET /users/{id} — получить одного пользователя по ID
pub async fn get_user(
    path: web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ApiError> {
    let user_id = path.into_inner();
    let pool_clone = pool.clone();
    let result = web::block(move || get_user_service(user_id, pool_clone.get_ref())).await;

    match result {
        Ok(inner) => match inner {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(api_err) => Err(api_err),
        },
        Err(_) => Err(ApiError::InternalError),
    }
}

/// POST /users — создать нового пользователя
pub async fn create_user(
    json: web::Json<NewUser>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ApiError> {
    let new_user = json.into_inner();
    let pool_clone = pool.clone();
    let result = web::block(move || create_user_service(new_user, pool_clone.get_ref())).await;

    match result {
        Ok(inner) => match inner {
            Ok(user) => Ok(HttpResponse::Created().json(user)),
            Err(api_err) => Err(api_err),
        },
        Err(_) => Err(ApiError::InternalError),
    }
}

/// PUT /users/{id} — обновить существующего пользователя
pub async fn update_user(
    path: web::Path<i32>,
    json: web::Json<UpdateUser>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ApiError> {
    let user_id = path.into_inner();
    let upd_data = json.into_inner();
    let pool_clone = pool.clone();
    let result =
        web::block(move || update_user_service(user_id, upd_data, pool_clone.get_ref())).await;

    match result {
        Ok(inner) => match inner {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(api_err) => Err(api_err),
        },
        Err(_) => Err(ApiError::InternalError),
    }
}

/// DELETE /users/{id} — удалить пользователя
pub async fn delete_user(
    path: web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ApiError> {
    let user_id = path.into_inner();
    let pool_clone = pool.clone();
    let result =
        web::block(move || delete_user_service(user_id, pool_clone.get_ref())).await;

    match result {
        Ok(inner) => match inner {
            // Если в сервисе всё успешно (Ok(count)), возвращаем 204 No Content
            Ok(_) => Ok(HttpResponse::NoContent().finish()),
            Err(api_err) => Err(api_err),
        },
        Err(_) => Err(ApiError::InternalError),
    }
}
