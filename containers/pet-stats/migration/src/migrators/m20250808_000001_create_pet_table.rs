use sea_orm::{ActiveEnum, DbBackend, DeriveActiveEnum, EnumIter, Schema, TransactionTrait};
use sea_orm_migration::prelude::*;

use super::{m20250121_000001_create_user_table::Users, utils::current_timestamp_col};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20250808_000001_create_pet_table"
    }
}

#[derive(EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "date_duration_type")]
pub enum BirthdayPrecisionType {
    #[sea_orm(string_value = "FullDate")]
    FullDate,
    #[sea_orm(string_value = "Month")]
    Month,
    #[sea_orm(string_value = "Year")]
    Year,
}

#[derive(EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "feed_duration_type")]
pub enum FeedDurationType {
    #[sea_orm(string_value = "Day")]
    Day,
    #[sea_orm(string_value = "Week")]
    Week,
    #[sea_orm(string_value = "Month")]
    Month,
}

#[derive(EnumIter, DeriveActiveEnum, Eq, Debug, Clone, PartialEq)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "pet_sex_type")]
pub enum PetSexType {
    #[sea_orm(string_value = "Male")]
    Male,
    #[sea_orm(string_value = "Female")]
    Female,
    #[sea_orm(string_value = "Other")]
    Other,
}

#[derive(EnumIter, DeriveActiveEnum, Eq, Debug, Clone, PartialEq)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "pet_species_type")]
pub enum PetSpeciesType {
    #[sea_orm(string_value = "Dog")]
    Dog,
    #[sea_orm(string_value = "Cat")]
    Cat,
    #[sea_orm(string_value = "Fish")]
    Fish,
    #[sea_orm(string_value = "Lizard")]
    Lizard,
    #[sea_orm(string_value = "Turtle")]
    Turtle,
    #[sea_orm(string_value = "Snake")]
    Snake,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let psql_db = DbBackend::Postgres;
        let schema = Schema::new(psql_db);
        let db = manager.get_connection();
        let transaction = db.begin().await?;

        // Add new enums.
        manager
            .create_type(schema.create_enum_from_active_enum::<BirthdayPrecisionType>())
            .await?;

        manager
            .create_type(schema.create_enum_from_active_enum::<FeedDurationType>())
            .await?;
        manager
            .create_type(schema.create_enum_from_active_enum::<PetSexType>())
            .await?;
        manager
            .create_type(schema.create_enum_from_active_enum::<PetSpeciesType>())
            .await?;

        // Pets table.
        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(Pets::Table)
                    .col(
                        ColumnDef::new(Pets::Id)
                            .integer()
                            .not_null()
                            .extra("GENERATED ALWAYS AS IDENTITY"),
                    )
                    .primary_key(Index::create().col(Pets::Id))
                    .col(ColumnDef::new(Pets::UserId).integer().not_null())
                    .col(ColumnDef::new(Pets::Name).string_len(255).not_null())
                    .col(
                        ColumnDef::new(Pets::Sex)
                            .custom(PetSexType::name())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Pets::Species)
                            .custom(PetSpeciesType::name())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Pets::Birthday)
                            .custom("daterange")
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Pets::BirthdayPrecision)
                            .custom(BirthdayPrecisionType::name())
                            .not_null(),
                    )
                    .col(ColumnDef::new(Pets::FeedCount).integer().not_null())
                    .col(
                        ColumnDef::new(Pets::FeedCountPer)
                            .custom(FeedDurationType::name())
                            .not_null()
                            .default(Expr::value(FeedDurationType::Day)),
                    )
                    .col(ColumnDef::new(Pets::Weight).float().null())
                    .col(
                        ColumnDef::new(Pets::IsDisabled)
                            .boolean()
                            .not_null()
                            .default(Expr::value(false)),
                    )
                    .col(current_timestamp_col(Pets::CreatedAt))
                    .col(current_timestamp_col(Pets::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_pets_user_id")
                            .from(Pets::Table, Pets::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Feed record table.
        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(FeedRecords::Table)
                    .col(
                        ColumnDef::new(FeedRecords::Id)
                            .integer()
                            .primary_key()
                            .extra("GENERATED ALWAYS AS IDENTITY"),
                    )
                    .col(ColumnDef::new(FeedRecords::PetId).integer().not_null())
                    .col(ColumnDef::new(FeedRecords::Amount).integer().null())
                    .col(current_timestamp_col(FeedRecords::CreatedAt))
                    .col(current_timestamp_col(FeedRecords::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_pet_feeds_pet_id")
                            .from(FeedRecords::Table, FeedRecords::PetId)
                            .to(Pets::Table, Pets::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        // Work goals table.
        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(WorkGoals::Table)
                    .col(
                        ColumnDef::new(WorkGoals::Id)
                            .integer()
                            .primary_key()
                            .extra("GENERATED ALWAYS AS IDENTITY"),
                    )
                    .col(ColumnDef::new(WorkGoals::PetId).integer().not_null())
                    .col(
                        ColumnDef::new(WorkGoals::Time)
                            // Approval HH:MM or MM or HH only.
                            .interval(Some(PgInterval::HourToMinute), None),
                    )
                    .col(ColumnDef::new(WorkGoals::Count).integer().null())
                    .col(current_timestamp_col(WorkGoals::CreatedAt))
                    .col(current_timestamp_col(WorkGoals::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_pet_work_goals_pet_id")
                            .from(WorkGoals::Table, WorkGoals::PetId)
                            .to(Pets::Table, Pets::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Work record table.
        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(WorkRecords::Table)
                    .col(
                        ColumnDef::new(WorkRecords::Id)
                            .integer()
                            .primary_key()
                            .extra("GENERATED ALWAYS AS IDENTITY"),
                    )
                    .col(ColumnDef::new(WorkRecords::PetId).integer().not_null())
                    .col(
                        ColumnDef::new(WorkRecords::Time)
                            .interval(Some(PgInterval::HourToMinute), None)
                            .null(),
                    )
                    .col(ColumnDef::new(WorkRecords::DistanceM).integer().null())
                    .col(current_timestamp_col(WorkRecords::CreatedAt))
                    .col(current_timestamp_col(WorkRecords::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_pet_work_records_pet_id")
                            .from(WorkRecords::Table, WorkRecords::PetId)
                            .to(Pets::Table, Pets::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        transaction.commit().await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Err(DbErr::Migration("We Don't Do That Here".to_owned()))
    }
}

#[derive(Iden)]
pub enum Pets {
    Table,
    Id,
    Name,
    Sex,
    Species,
    Birthday,
    BirthdayPrecision,
    FeedCount,
    FeedCountPer,
    Weight,
    UserId,
    IsDisabled,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
pub enum FeedRecords {
    Table,
    Id,
    PetId,
    Amount,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
pub enum WorkGoals {
    Table,
    Id,
    PetId,
    Time,
    Count,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
pub enum WorkRecords {
    Table,
    Id,
    PetId,
    Time,
    DistanceM,
    CreatedAt,
    UpdatedAt,
}
