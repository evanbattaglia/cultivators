use sea_orm::{
    sqlx::{sqlite::SqliteConnectOptions, ConnectOptions, Pool, Sqlite},
    DatabaseConnection, SqlxError, SqlxSqliteConnector,
};

use migration::{Migrator, MigratorTrait};

use log::LevelFilter::Warn;
use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbConnectionError {
    #[error("Failed to connect to db: {0}")]
    ConnectionError(#[from] SqlxError),
    #[error("Failed to run migrations: {0}")]
    MigrationError(#[from] sea_orm::DbErr),
}

pub async fn create_db_connection() -> Result<DatabaseConnection, DbConnectionError> {
    let filename =
        std::env::var("CULTIVATORS_SQLITE_FILE").expect("CULTIVATORS_SQLITE_FILE must be set");
    let conn_opts = SqliteConnectOptions::new()
        .filename(filename)
        .create_if_missing(true)
        .disable_statement_logging()
        .log_slow_statements(Warn, Duration::from_millis(100));

    let pool = Pool::<Sqlite>::connect_with(conn_opts).await?;
    let conn = SqlxSqliteConnector::from_sqlx_sqlite_pool(pool);

    Migrator::up(&conn, None).await?;

    Ok(conn)
}
