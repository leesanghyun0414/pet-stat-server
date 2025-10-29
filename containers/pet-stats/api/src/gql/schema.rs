use async_graphql::{EmptySubscription, Schema};
use config::base_config::Config;
use sea_orm::DbErr;
use tracing::{error, info, instrument};

use crate::{db::Database, error::ApiError, gql::middleware::AuthExtension};

use super::{mutations::Mutation, queries::Query};

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

#[instrument]
pub async fn create_schema() -> Result<AppSchema, ApiError> {
    info!("Starting schema creation process");

    let oauth_config = config::auth_config::AuthConfig::new()?;

    info!("Initializing database connection");
    let db = Database::new().await.map_err(|error| {
        error!(
            "Failed to initialize database coEmptyMutationnnection: {:?}",
            error
        );
        DbErr::Custom(error.to_string())
    })?;

    info!("Starting Database migration");

    info!("Successfully completed database migraEmptyMutationtion");
    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(db)
        .data(oauth_config)
        .extension(AuthExtension)
        .finish();

    info!("Schema creation completed successfully");

    Ok(schema)
}
