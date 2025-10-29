use async_graphql::{Enum, InputObject, SimpleObject};
use chrono::NaiveDate;
use entity::entities::{pets, users};
use sea_orm::{prelude::DateTimeWithTimeZone, ActiveValue::Set};

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

#[derive(Debug, SimpleObject)]
pub struct Pet {
    pub id: i32,
    pub name: String,
    pub feed_count: Option<i32>,
    pub sex: PetSexType,
    pub species: PetSpeciesType,
    pub feed_count_per: Option<FeedDurationType>,
    pub birthday: NaiveDate,
    pub birthday_precision: DateDurationType,
    pub weight: Option<f32>,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Debug, InputObject)]
pub(crate) struct NewPetInput {
    pub user_id: i32,
    pub name: String,
    pub sex: PetSexType,
    pub species: PetSpeciesType,
    #[graphql(default = 1)]
    pub feed_count: i32,
    pub feed_count_per: Option<FeedDurationType>,
    pub birthday: NaiveDate,
    pub birthday_precision: DateDurationType,
    pub weight: Option<f32>,
}

impl From<NewPetInput> for pets::ActiveModel {
    fn from(value: NewPetInput) -> Self {
        pets::ActiveModel {
            user_id: Set(value.user_id),
            name: Set(value.name),
            sex: Set(value.sex.into()),
            species: Set(value.species.into()),
            birthday: Set(value.birthday),
            birthday_precision: Set(value.birthday_precision.into()),
            feed_count: Set(Some(value.feed_count)),
            feed_count_per: Set(value.feed_count_per.map(|v| v.into())),
            ..Default::default()
        }
    }
}

impl From<pets::Model> for Pet {
    fn from(value: pets::Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            sex: PetSexType::from(value.sex),
            species: PetSpeciesType::from(value.species),
            feed_count: value.feed_count,
            feed_count_per: value.feed_count_per.map(FeedDurationType::from),
            birthday: value.birthday,
            birthday_precision: DateDurationType::from(value.birthday_precision),
            weight: value.weight,
            created_at: value.created_at,
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
#[graphql(remote = "entity::entities::sea_orm_active_enums::PetSpeciesType")]
pub enum PetSpeciesType {
    Dog,
    Cat,
    Fish,
    Lizard,
    Turtle,
    Snake,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
#[graphql(remote = "entity::entities::sea_orm_active_enums::PetSexType")]
pub enum PetSexType {
    Male,
    Female,
    Other,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
#[graphql(remote = "entity::entities::sea_orm_active_enums::DateDurationType")]
pub enum DateDurationType {
    FullDate,
    Month,
    Year,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
#[graphql(remote = "entity::entities::sea_orm_active_enums::FeedDurationType")]
pub enum FeedDurationType {
    Day,
    Week,
    Month,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
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

#[derive(InputObject, Debug)]
pub struct OauthSignInInput {
    pub id_token: String,
    pub provider_type: ProviderType,
}

#[derive(SimpleObject, Debug)]
pub struct SignOutPayload {
    pub success: bool,
    pub message: String,
}

#[derive(SimpleObject)]
pub struct TokenRotationPayload {
    pub access_token: String,
    pub refresh_token: String,
}
