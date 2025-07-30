// src/db.rs

// Diesel re-export’ит r2d2, поэтому Pool и ConnectionManager берём из diesel::r2d2
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
// Но ошибку берем из "чистого" r2d2, если у вас в Cargo.toml есть r2d2 = "0.8"
use r2d2::Error as DbPoolError;

use tracing::info;

/// Тип пула соединений. Diesel::r2d2::Pool<C> == r2d2::Pool<C>, но Error будет "чистым" r2d2::Error
pub type DbPool = Pool<ConnectionManager<PgConnection>>;

/// Обёртка над PooledConnection, чтобы в хэндлерах можно было делать &*conn
pub struct DbConn(pub PooledConnection<ConnectionManager<PgConnection>>);

impl std::ops::Deref for DbConn {
    type Target = PgConnection;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Инициализирует пул соединений.
/// Важно: Pool::builder().build(...) возвращает Err(r2d2::Error), 
/// поэтому сигнатура у нас -> Result<DbPool, r2d2::Error>.
pub fn init_pool(database_url: &str) -> Result<DbPool, DbPoolError> {
    // 1. Менеджер для PgConnection
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    // 2. Строим пул. Если что-то пошло не так, build(manager) вернёт Err(r2d2::Error),
    //    и оператор `?` пробросит эту ошибку наружу.
    let pool = Pool::builder().build(manager)?;
    info!("Initialized DB pool with URL: {}", database_url);
    Ok(pool)
}
