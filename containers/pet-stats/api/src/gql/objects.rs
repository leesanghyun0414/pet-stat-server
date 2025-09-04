use async_graphql::{Enum, InputObject, SimpleObject};
use chrono::{DateTime, FixedOffset};
use entity::entities::{user_tokens, users};

#[derive(Debug, Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(remote = "entity::entities::sea_orm_active_enums::LoginType")]
pub enum LoginType {
    Oauth,
    Local,
}

#[derive(Debug, SimpleObject)]
pub struct User {
    pub id: i32,
    pub email: Option<String>,
    pub login_type: LoginType,
}

impl From<users::Model> for User {
    fn from(entity: users::Model) -> Self {
        Self {
            id: entity.id,
            email: entity.email,
            login_type: LoginType::from(entity.login_type),
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(remote = "entity::entities::sea_orm_active_enums::ProviderType")]
pub enum ProviderType {
    Google,
    Meta,
    Apple,
}

#[derive(SimpleObject)]
pub struct OauthPayload {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(InputObject)]
pub struct OauthSignInInput {
    pub id_token: String,
    pub provider_type: ProviderType,
}
