// src/models/user.rs

use crate::schema::users;           // Таблица `users` из schema.rs
use chrono::NaiveDateTime;          // Для поля created_at
use diesel::{AsChangeset, Insertable, Queryable}; // derive-макросы Diesel-а
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Модель, возвращаемая из базы
#[derive(Debug, Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}

/// Структура для вставки нового пользователя
#[derive(Debug, Insertable, Deserialize, Validate)]
#[diesel(table_name = users)]
pub struct NewUser {
    #[validate(length(min = 3, message = "Username must be at least 3 characters"))]
    pub username: String,

    #[validate(email(message = "Email must be a valid email"))]
    pub email: String,
}

/// Структура для обновления существующего пользователя (PATCH semantics)
#[derive(Debug, AsChangeset, Deserialize, Validate)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    #[validate(length(min = 3, message = "Username must be at least 3 characters"))]
    pub username: Option<String>,

    #[validate(email(message = "Email must be a valid email"))]
    pub email: Option<String>,
}
