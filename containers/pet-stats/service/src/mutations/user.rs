use chrono::{Duration, Local};
use entity::entities::sea_orm_active_enums::{LoginType, ProviderType};
use entity::entities::user_tokens::{self, Column as C, Entity as UserTokens, Model};
use entity::entities::{oauth_accounts, users};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbConn, DbErr, EntityTrait, IntoActiveModel, QueryFilter,
    QuerySelect, TransactionTrait,
};
use tracing::{error, info, instrument};

pub struct UserMutation;

impl UserMutation {
    #[instrument(skip(db), fields())]
    pub async fn create_oauth_user(
        db: &DbConn,
        email: Option<String>,
        provider_type: ProviderType,
        provider_user_id: String,
    ) -> Result<users::Model, DbErr> {
        let txn = db.begin().await?;
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

        txn.commit().await?;

        Ok(new_user)
    }

    #[instrument(skip(db), fields())]
    pub async fn store_refresh_token(
        db: &DbConn,
        user_id: i32,
        token_hash: &[u8; 32],
    ) -> Result<Model, DbErr> {
        let expires = (Local::now() + Duration::days(60)).with_timezone(Local::now().offset());

        let user_token = user_tokens::ActiveModel {
            user_id: Set(user_id),
            refresh_token: Set(token_hash.to_vec()),
            expires_at: Set(expires),
            ..Default::default()
        }
        .insert(db)
        .await
        .inspect(|m| info!("A refresh stored successfully {:?}", m.user_id))
        .inspect_err(|e| error!("Error store refresh token - {:?}", e))?;

        Ok(user_token)
    }

    #[instrument(skip(db), fields())]
    pub async fn rotate_refresh_token(
        db: &DbConn,
        old_hash: &[u8; 32],
        user_id: i32,
        new_hash: &[u8; 32],
    ) -> Result<Model, DbErr> {
        let txn = db.begin().await?;

        let Some(user_token) = UserTokens::find()
            .filter(C::RefreshToken.eq(old_hash.as_slice()))
            .lock_exclusive()
            .one(&txn)
            .await?
        else {
            txn.rollback().await?;
            return Err(DbErr::RecordNotFound("Record Not Found".to_owned()));
        };
        let mut user_token_am = user_token.into_active_model();
        user_token_am.revoked = Set(Some(true));
        user_token_am.update(&txn).await?;

        let expires = (Local::now() + Duration::days(60)).with_timezone(Local::now().offset());

        let new_user_token = user_tokens::ActiveModel {
            user_id: Set(user_id),
            refresh_token: Set(new_hash.to_vec()),
            expires_at: Set(expires),
            ..Default::default()
        }
        .insert(&txn)
        .await?;

        txn.commit().await?;
        Ok(new_user_token)
    }
}
