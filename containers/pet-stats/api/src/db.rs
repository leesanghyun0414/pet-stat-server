use config::db_config::PET_STAT_CENTRAL_DB_CONFIG;
use sea_orm::DatabaseConnection;
use tracing::instrument;

use crate::error::ApiError;

pub struct Database {
    pub connection: DatabaseConnection,
}

impl Database {
    #[instrument]
    pub async fn new() -> Result<Self, ApiError> {
        let db_config = &*PET_STAT_CENTRAL_DB_CONFIG;
        let connection = sea_orm::Database::connect(db_config.url.clone()).await?;

        Ok(Database { connection })
    }
    pub fn get_connection(&self) -> &DatabaseConnection {
        &self.connection
    }
}
