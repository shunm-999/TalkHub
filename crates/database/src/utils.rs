use std::ops::{Deref, DerefMut};

use deadpool::Runtime;
use diesel::result::Error;
use diesel::result::Error::QueryBuilderError;
use diesel::row::NamedRow;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::{
    pg::AsyncPgConnection,
    pooled_connection::deadpool::{Object as PooledConnection, Pool},
};

use talk_hub_domain::result::TalkHubResult;

pub type DieselError = Error;
pub type ActualDbPool = Pool<AsyncPgConnection>;

pub enum DbPool<'a> {
    Pool(&'a ActualDbPool),
    Conn(&'a mut AsyncPgConnection),
}

pub enum DbConn<'a> {
    Pool(PooledConnection<AsyncPgConnection>),
    Conn(&'a mut AsyncPgConnection),
}

pub async fn get_conn<'a, 'b: 'a>(pool: &'a mut DbPool<'b>) -> Result<DbConn<'a>, DieselError> {
    let conn = match pool {
        DbPool::Pool(pool) => {
            DbConn::Pool(pool.get().await.map_err(|e| QueryBuilderError(e.into()))?)
        }
        DbPool::Conn(conn) => DbConn::Conn(conn),
    };
    Ok(conn)
}

impl<'a> Deref for DbConn<'a> {
    type Target = AsyncPgConnection;

    fn deref(&self) -> &Self::Target {
        match self {
            DbConn::Pool(conn) => conn.deref(),
            DbConn::Conn(conn) => conn.deref(),
        }
    }
}

impl<'a> DerefMut for DbConn<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            DbConn::Pool(conn) => conn.deref_mut(),
            DbConn::Conn(conn) => conn.deref_mut(),
        }
    }
}

impl<'a> From<&'a mut AsyncPgConnection> for DbPool<'a> {
    fn from(value: &'a mut AsyncPgConnection) -> Self {
        DbPool::Conn(value)
    }
}

impl<'a, 'b: 'a> From<&'a mut DbConn<'b>> for DbPool<'a> {
    fn from(value: &'a mut DbConn<'b>) -> Self {
        DbPool::Conn(value.deref_mut())
    }
}

impl<'a> From<&'a ActualDbPool> for DbPool<'a> {
    fn from(value: &'a ActualDbPool) -> Self {
        DbPool::Pool(value)
    }
}

pub async fn build_db_pool(db_url: &String, pool_size: usize) -> TalkHubResult<ActualDbPool> {
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);
    let pool = Pool::builder(manager)
        .max_size(pool_size)
        .runtime(Runtime::Tokio1)
        .build()?;

    crate::migration::run(db_url)?;

    Ok(pool)
}
