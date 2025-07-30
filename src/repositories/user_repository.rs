// src/repositories/user_repository.rs

use crate::errors::ApiError;
use crate::models::user::{NewUser, UpdateUser, User};
use crate::schema::users::dsl::*;
use crate::db::DbPool;
use diesel::prelude::*; // для методов load, first, insert_into, delete, update, get_result, execute

/// Получить всех пользователей
pub fn get_all_users(pool: &DbPool) -> Result<Vec<User>, ApiError> {
    // Теперь pool.get()? возвращает r2d2::Error, который автоматически конвертируется в ApiError::PoolError
    let mut conn = pool.get()?;
    let items = users.load::<User>(&mut conn)?; // diesel::result::Error -> ApiError::DbError
    Ok(items)
}

/// Получить одного пользователя по ID
pub fn get_user_by_id(user_id: i32, pool: &DbPool) -> Result<User, ApiError> {
    let mut conn = pool.get()?;
    let user = users.filter(id.eq(user_id)).first::<User>(&mut conn)?;
    Ok(user)
}

/// Создать нового пользователя
pub fn create_user(new_user: NewUser, pool: &DbPool) -> Result<User, ApiError> {
    let mut conn = pool.get()?;
    let created: User = diesel::insert_into(users)
        .values(&new_user)
        .get_result(&mut conn)?;
    Ok(created)
}

/// Обновить существующего пользователя
pub fn update_user(user_id: i32, upd: UpdateUser, pool: &DbPool) -> Result<User, ApiError> {
    let mut conn = pool.get()?;
    let updated: User = diesel::update(users.filter(id.eq(user_id)))
        .set(&upd)
        .get_result(&mut conn)?;
    Ok(updated)
}

/// Удалить пользователя; вернуть число удалённых строк
pub fn delete_user(user_id: i32, pool: &DbPool) -> Result<usize, ApiError> {
    let mut conn = pool.get()?;
    let count = diesel::delete(users.filter(id.eq(user_id))).execute(&mut conn)?;
    Ok(count)
}
