use anyhow::Context;
use diesel::Connection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use talk_hub_domain::result::TalkHubResult;

const MIGRATION: EmbeddedMigrations = embed_migrations!();

pub(crate) fn run(db_url: &str) -> TalkHubResult<()> {
    let mut conn =
        diesel::PgConnection::establish(db_url).with_context(|| "Error connecting to database")?;
    let migrations = conn
        .pending_migrations(MIGRATION)
        .map_err(|e| anyhow::anyhow!("Couldn't determine pending migrations: {e}"))?;

    for migration in migrations.iter() {
        conn.run_migration(migration)
            .map_err(|e| anyhow::anyhow!("Couldn't run DB Migrations: {e}"))?;
    }

    Ok(())
}
