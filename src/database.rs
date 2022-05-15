use std::env;

use axum::Extension;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;

use crate::repos_impl::tweets::TweetsImpl;

pub type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

pub async fn layer() -> Extension<RepositoryProvider> {
    let database_url = env::var("DATABASE_URL").unwrap();
    let manager = PostgresConnectionManager::new_from_stringlike(database_url, NoTls).unwrap();
    let pool = Pool::builder().build(manager).await.unwrap();
    Extension(RepositoryProvider(pool))
}

#[derive(Clone)]
pub struct RepositoryProvider(ConnectionPool);

impl RepositoryProvider {
    pub fn tweets(&self) -> TweetsImpl {
        TweetsImpl { pool: &self.0 }
    }
}
