use crate::errors::ApiError;
use crate::models::user::{NewUser, UpdateUser, User};
use crate::repositories::user_repository;
use crate::db::DbPool;
use validator::Validate;
use serde_json::json;

pub fn get_users_service(pool: &DbPool) -> Result<Vec<User>, ApiError> {
    let users = user_repository::get_all_users(pool)?;
    Ok(users)
}

pub fn get_user_service(user_id: i32, pool: &DbPool) -> Result<User, ApiError> {
    let user = user_repository::get_user_by_id(user_id, pool)?;
    Ok(user)
}

pub fn create_user_service(new_user: NewUser, pool: &DbPool) -> Result<User, ApiError> {
    if let Err(errors) = new_user.validate() {
        let errors_json = errors
            .field_errors()
            .iter()
            .map(|(field, errs)| {
                let messages: Vec<String> = errs.iter().filter_map(|e| e.message.clone().map(|m| m.to_string())).collect();
                (field.to_string(), json!(messages))
            })
            .collect::<serde_json::Map<String, serde_json::Value>>();
        return Err(ApiError::ValidationJsonError(json!(errors_json)));
    }
    let user = user_repository::create_user(new_user, pool)?;
    Ok(user)
}

pub fn update_user_service(
    user_id: i32,
    upd: UpdateUser,
    pool: &DbPool,
) -> Result<User, ApiError> {
    if let Err(errors) = upd.validate() {
        let errors_json = errors
            .field_errors()
            .iter()
            .map(|(field, errs)| {
                let messages: Vec<String> = errs.iter().filter_map(|e| e.message.clone().map(|m| m.to_string())).collect();
                (field.to_string(), json!(messages))
            })
            .collect::<serde_json::Map<String, serde_json::Value>>();
        return Err(ApiError::ValidationJsonError(json!(errors_json)));
    }
    let user = user_repository::update_user(user_id, upd, pool)?;
    Ok(user)
}

pub fn delete_user_service(user_id: i32, pool: &DbPool) -> Result<usize, ApiError> {
    let count = user_repository::delete_user(user_id, pool)?;
    if count == 0 {
        Err(ApiError::NotFound)
    } else {
        Ok(count)
    }
}
