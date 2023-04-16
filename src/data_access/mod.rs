pub mod errors;
pub mod repository;
pub mod schema;

use crate::settings::Settings;

use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
pub use errors::DataAccessError;

#[derive(Clone)]
pub struct PgConnectionPool(std::sync::Arc<Pool<AsyncPgConnection>>);

impl PgConnectionPool {
    pub fn new(pool: Pool<AsyncPgConnection>) -> Self {
        Self(std::sync::Arc::new(pool))
    }
}

impl AsRef<std::sync::Arc<Pool<AsyncPgConnection>>> for PgConnectionPool {
    fn as_ref(&self) -> &std::sync::Arc<Pool<AsyncPgConnection>> {
        &self.0
    }
}

pub async fn build_connection_pool(
    settings: &Settings,
) -> Result<PgConnectionPool, DataAccessError> {
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(
        &settings.database.uri,
    );
    let pool = Pool::builder(config)
        .build()
        .map_err(|err| DataAccessError::ConnectionError(err.to_string()))?;
    Ok(PgConnectionPool::new(pool))
}
