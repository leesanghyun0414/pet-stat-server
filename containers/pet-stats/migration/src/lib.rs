mod migrators;

use sea_orm::Database;
pub use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(
            migrators::m20250121_000001_create_user_table::Migration,
        )]
    }
}

pub async fn run() -> Result<(), DbErr> {
    let db = Database::connect("postgres://dev:dev@psql/pet_stat").await?;
    Migrator::up(&db, None).await?;

    Ok(())
}
