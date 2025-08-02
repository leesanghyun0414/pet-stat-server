use ::entity::entities::{
    user_tokens, user_tokens::Entity as UserTokens, users, users::Entity as Users,
};
use entity::entities::oauth_accounts;
use sea_orm::{
    sea_query::token, ColumnTrait, DbConn, DbErr, EntityTrait, Iterable, JoinType, QueryFilter,
    QuerySelect, RelationTrait,
};
use tracing::instrument;

pub struct UserQuery;

impl UserQuery {
    #[instrument(skip(db), fields())]
    pub async fn users(db: &DbConn) -> Result<Vec<users::Model>, DbErr> {
        Users::find().all(db).await
    }

    #[instrument(skip(db), fields(user_id = id))]
    pub async fn user_by_id(db: &DbConn, id: i32) -> Result<users::Model, DbErr> {
        let user = Users::find_by_id(id).one(db).await?;
        user.ok_or_else(|| DbErr::RecordNotFound(format!("User Not found ID {}", id)))
    }

    #[instrument(skip(db), fields(user_id = id))]
    pub async fn user_with_token(
        db: &DbConn,
        id: i32,
    ) -> Result<Option<(users::Model, Option<user_tokens::Model>)>, DbErr> {
        Users::find_by_id(id)
            .find_also_related(UserTokens)
            .into_model()
            .one(db)
            .await
    }

    #[instrument(skip(db), fields(provider_user_id = id))]
    pub async fn user_by_provider_user_id(db: &DbConn, id: String) -> Result<users::Model, DbErr> {
        let user = users::Entity::find()
            .join(JoinType::InnerJoin, users::Relation::OauthAccounts.def()) // JOIN 사용
            .filter(oauth_accounts::Column::ProviderUserId.eq(id.clone()))
            .select_only()
            .columns(users::Column::iter())
            .into_model::<users::Model>() // WHERE 절 필터링
            .one(db)
            .await?
            .ok_or_else(|| DbErr::RecordNotFound("User Not Found".to_owned()))?;

        Ok(user)
    }

    #[instrument(skip(db), fields())]
    pub async fn user_token_by_token_hash(
        db: &DbConn,
        token_hash: &[u8; 32],
    ) -> Result<user_tokens::Model, DbErr> {
        let user_token = user_tokens::Entity::find()
            .filter(user_tokens::Column::RefreshToken.eq(token_hash.as_slice()))
            .filter(user_tokens::Column::Revoked.eq(false))
            .one(db)
            .await?
            .ok_or_else(|| DbErr::RecordNotFound("Token Not Found".to_owned()))?;
        Ok(user_token)
    }
}
