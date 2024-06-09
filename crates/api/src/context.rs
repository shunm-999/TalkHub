use talk_hub_database::utils::{ActualDbPool, DbPool};

#[derive(Clone)]
pub struct TalkHubContext {
    pool: ActualDbPool,
}

impl TalkHubContext {
    pub fn create(pool: ActualDbPool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> DbPool<'_> {
        DbPool::Pool(&self.pool)
    }

    pub fn inner_pool(&self) -> &ActualDbPool {
        &self.pool
    }
}
