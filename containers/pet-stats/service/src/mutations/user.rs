use chrono::{Duration, Local};
use entity::entities::sea_orm_active_enums::{LoginType, ProviderType};
use entity::entities::user_tokens::{self, Column as C, Entity as UserTokens, Model};
use entity::entities::{oauth_accounts, users};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseTransaction, DbConn, DbErr,
    EntityTrait, IntoActiveModel, QueryFilter, QueryOrder, QuerySelect, TransactionTrait,
};
use tracing::{debug, error, info, instrument};

use crate::utils::{commit_transaction, start_transaction};

pub struct UserMutation;

impl UserMutation {
    #[instrument(skip(db), fields())]
    pub async fn create_oauth_user(
        db: &DbConn,
        email: Option<String>,
        provider_type: ProviderType,
        provider_user_id: String,
    ) -> Result<users::Model, DbErr> {
        info!(
            "Starting OAuth user creation for provider: {:?}",
            provider_type
        );
        let txn = start_transaction(db).await?;
        let new_user = users::ActiveModel {
            email: Set(email),
            login_type: Set(LoginType::Oauth),
            ..Default::default()
        }
        .insert(&txn)
        .await?;

        let _oauth_account = oauth_accounts::ActiveModel {
            user_id: Set(new_user.id),
            provider_user_id: Set(provider_user_id),
            provider_type: Set(provider_type),
            ..Default::default()
        }
        .insert(&txn)
        .await?;
        commit_transaction(txn).await?;

        Ok(new_user)
    }

    #[instrument(skip(db, token_hash), fields())]
    pub async fn store_refresh_token(
        db: &DbConn,
        user_id: i32,
        token_hash: &[u8; 32],
    ) -> Result<Model, DbErr> {
        let expires = (Local::now() + Duration::days(60)).with_timezone(Local::now().offset());
        info!("Starting Store refresh token.");
        let user_token = user_tokens::ActiveModel {
            user_id: Set(user_id),
            refresh_token: Set(token_hash.to_vec()),
            expires_at: Set(expires),
            ..Default::default()
        }
        .insert(db)
        .await
        .inspect(|m| info!("A refresh stored successfully {:?}", m.created_at))
        .inspect_err(|e| error!("Error store refresh token - {:?}", e))?;

        Ok(user_token)
    }

    #[instrument(skip(txn), fields())]
    pub async fn revoke_refresh_token<T>(
        txn: &T,
        hash: &[u8; 32],
    ) -> Result<user_tokens::Model, DbErr>
    where
        T: ConnectionTrait,
    {
        let Some(user_token) = UserTokens::find()
            .filter(C::RefreshToken.eq(hash.as_slice()))
            .lock_exclusive()
            .one(txn)
            .await?
        else {
            return Err(DbErr::RecordNotFound("Not found".to_string()));
        };

        let mut am = user_token.into_active_model();
        am.revoked = Set(Some(true));
        am.updated_at = Set(Local::now().fixed_offset());
        let updated = am.update(txn).await?;

        Ok(updated)
    }

    #[instrument(skip(db), fields())]
    pub async fn rotate_refresh_token(
        db: &DbConn,
        old_hash: &[u8; 32],
        new_hash: &[u8; 32],
    ) -> Result<Model, DbErr> {
        let txn = start_transaction(db).await?;

        let old_token = Self::revoke_refresh_token(&txn, old_hash).await?;
        let user_id = old_token.user_id;
        info!("Old token revoked status: {:?}", old_token.revoked);
        let expires = (Local::now() + Duration::days(60)).with_timezone(Local::now().offset());

        let new_user_token = user_tokens::ActiveModel {
            user_id: Set(user_id),
            refresh_token: Set(new_hash.to_vec()),
            expires_at: Set(expires),
            ..Default::default()
        }
        .insert(&txn)
        .await?;

        commit_transaction(txn).await?;

        Ok(new_user_token)
    }
}
