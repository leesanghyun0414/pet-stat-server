use crate::db::Database;
use crate::gql::guards::AuthGuard;
use crate::gql::objects::{DefaultPet, DeleteObjectPayload, NewPetInput, Pet, UpdatePetInput};
use crate::gql::utils::verified_claims_from_ctx;
use async_graphql::Result;
use async_graphql::{Context, Object};
use entity::entities::pets;
use sea_orm::ActiveValue::Set;
use service::mutations::pet::PetMutationService;
use tracing::{error, info, instrument};

#[derive(Default)]
pub struct PetMutation;

#[Object]
impl PetMutation {
    #[graphql(guard = "AuthGuard")]
    #[instrument(skip(self, input, ctx))]
    pub async fn add_pet(&self, ctx: &Context<'_>, input: NewPetInput) -> Result<Pet> {
        let db = ctx.data::<Database>()?;
        let conn = db.get_connection();

        let claims = verified_claims_from_ctx(ctx)?;

        let mut active_model = pets::ActiveModel::from(input);
        active_model.user_id = Set(claims.sub);

        let pet = PetMutationService::add_pet(conn, active_model).await?;

        Ok(Pet::from(pet))
    }

    #[graphql(guard = "AuthGuard")]
    #[instrument(skip(self, ctx))]
    pub async fn remove_pet(&self, ctx: &Context<'_>, pet_id: i32) -> Result<DeleteObjectPayload> {
        let db = ctx.data::<Database>()?;
        let conn = db.get_connection();

        let claims = verified_claims_from_ctx(ctx)?;

        let removed_pet = PetMutationService::remove_pet(conn, pet_id).await?;

        if removed_pet.rows_affected == 1 {
            Ok(DeleteObjectPayload::success_response(claims.sub))
        } else {
            Ok(DeleteObjectPayload::empty_response())
        }
    }

    #[graphql(guard = "AuthGuard")]
    #[instrument(skip(self, ctx))]
    pub async fn update_pet(&self, ctx: &Context<'_>, input: UpdatePetInput) -> Result<Pet> {
        let db = ctx.data::<Database>()?;
        let conn = db.get_connection();

        let claims = verified_claims_from_ctx(ctx)?;

        let mut pet = pets::ActiveModel::from(input);
        pet.user_id = Set(claims.sub);

        let updated_pet = PetMutationService::update_pet(conn, pet).await?;

        error!("{:?}", updated_pet);
        Ok(Pet::from(updated_pet))
    }
}
