use talk_hub_database::utils::ActualDbPool;

#[derive(Clone)]
pub struct TalkHubContext {
    pool: ActualDbPool,
}

impl TalkHubContext {
    pub fn create(pool: ActualDbPool) -> Self {
        Self { pool }
    }
}
