use crate::gql::objects::Pet;
use crate::gql::utils::verified_claims_from_ctx;
use crate::{db::Database, gql::guards::AuthGuard};
use async_graphql::{Context, Object, Result};
use service::queries::pet::PetQuery as ServicePetQuery;
use tracing::instrument;

#[derive(Default)]
pub struct PetQuery;

#[Object]
impl PetQuery {
    #[graphql(guard = "AuthGuard")]
    #[instrument(skip(self, ctx))]
    async fn get_pets(&self, ctx: &Context<'_>) -> Result<Vec<Pet>> {
        let db = ctx.data::<Database>()?;
        let conn = db.get_connection();

        let claims = verified_claims_from_ctx(ctx)?;

        let pets = ServicePetQuery::get_pets_by_user_id(conn, claims.sub).await?;

        Ok(pets.into_iter().map(Pet::from).collect())
    }

    #[graphql(guard = "AuthGuard")]
    #[instrument(skip(self, ctx))]
    async fn get_pet(&self, ctx: &Context<'_>, pet_id: i32) -> Result<Pet> {
        let db = ctx.data::<Database>()?;
        let conn = db.get_connection();

        verified_claims_from_ctx(ctx)?;

        let pet = ServicePetQuery::get_pet_by_id(conn, pet_id).await?;

        Ok(Pet::from(pet))
    }

    #[graphql(guard = "AuthGuard")]
    #[instrument(skip(self, ctx))]
    async fn count_pets(&self, ctx: &Context<'_>) -> Result<u64> {
        let db = ctx.data::<Database>()?;
        let conn = db.get_connection();

        let claims = verified_claims_from_ctx(ctx)?;

        let pet_count = ServicePetQuery::count_pets_by_user_id(conn, claims.sub).await?;

        Ok(pet_count)
    }
}
