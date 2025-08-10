use sea_orm::{ActiveEnum, DbBackend, DeriveActiveEnum, EnumIter, Schema, TransactionTrait};
use sea_orm_migration::prelude::*;

use super::utils::current_timestamp_col;

#[derive(EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "login_type")]
pub enum LoginType {
    #[sea_orm(string_value = "Oauth")]
    Oauth,
    #[sea_orm(string_value = "Local")]
    Local,
}

#[derive(EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "provider_type")]
pub enum ProviderType {
    #[sea_orm(string_value = "Google")]
    Google,
    #[sea_orm(string_value = "Meta")]
    Meta,
    #[sea_orm(string_value = "Apple")]
    Apple,
}

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20250121_000001_create_user_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let psql_db = DbBackend::Postgres;
        let schema = Schema::new(psql_db);
        let db = manager.get_connection();
        let transaction = db.begin().await?;
        manager
            .create_type(schema.create_enum_from_active_enum::<LoginType>())
            .await?;
        manager
            .create_type(schema.create_enum_from_active_enum::<ProviderType>())
            .await?;
        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(Users::Table)
                    .col(
                        ColumnDef::new(Users::Id)
                            .integer()
                            .not_null()
                            .extra("GENERATED ALWAYS AS IDENTITY".to_owned())
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Users::Email).string_len(255).null())
                    .col(ColumnDef::new(Users::PasswordHash).string_len(255).null())
                    .col(
                        ColumnDef::new(Users::LoginType)
                            .custom(LoginType::name())
                            .not_null(),
                    )
                    .col(current_timestamp_col(Users::CreatedAt))
                    .col(current_timestamp_col(Users::UpdatedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(OauthAccounts::Table)
                    .col(ColumnDef::new(OauthAccounts::UserId).integer().not_null())
                    .col(
                        ColumnDef::new(OauthAccounts::Id)
                            .integer()
                            .not_null()
                            .extra("GENERATED ALWAYS AS IDENTITY".to_owned())
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(OauthAccounts::ProviderUserId)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(ColumnDef::new(OauthAccounts::IdToken).string().null())
                    .col(
                        ColumnDef::new(OauthAccounts::ExtraData)
                            .json_binary()
                            .null(),
                    )
                    .col(current_timestamp_col(OauthAccounts::CreatedAt))
                    .col(current_timestamp_col(OauthAccounts::UpdatedAt))
                    .col(
                        ColumnDef::new(OauthAccounts::ProviderType)
                            .custom(ProviderType::name())
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_oauth_accounts_user_id")
                            .from(OauthAccounts::Table, OauthAccounts::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .if_not_exists()
                    .table(UserTokens::Table)
                    .col(
                        ColumnDef::new(UserTokens::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                            .extra("GENERATED ALWAYS AS IDENTITY".to_owned()),
                    )
                    .col(ColumnDef::new(UserTokens::UserId).integer().not_null())
                    .col(ColumnDef::new(UserTokens::DeviceId).string_len(255).null())
                    .col(
                        ColumnDef::new(UserTokens::RefreshToken)
                            .var_binary(32)
                            .not_null(),
                    )
                    .col(current_timestamp_col(UserTokens::CreatedAt))
                    .col(current_timestamp_col(UserTokens::UpdatedAt))
                    .col(
                        ColumnDef::new(UserTokens::ExpiresAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserTokens::Revoked)
                            .boolean()
                            .default(Expr::value(false)),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_tokens_user_id")
                            .from(UserTokens::Table, UserTokens::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-user-device")
                    .table(UserTokens::Table)
                    .col(UserTokens::UserId)
                    .col(UserTokens::RefreshToken)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-refresh-token")
                    .table(UserTokens::Table)
                    .col(UserTokens::RefreshToken)
                    .unique()
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
pub enum Users {
    Table,
    Id,
    Email,
    PasswordHash,
    LoginType,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
pub enum OauthAccounts {
    Table,
    Id,
    UserId,
    ProviderType,
    ProviderUserId,
    IdToken,
    ExtraData,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
pub enum UserTokens {
    Table,
    Id,
    UserId,
    DeviceId,
    CreatedAt,
    UpdatedAt,
    ExpiresAt,
    RefreshToken,
    Revoked,
}
