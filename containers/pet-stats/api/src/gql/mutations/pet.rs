use crate::db::Database;
use crate::gql::objects::{NewPetInput, Pet};
use async_graphql::Result;
use async_graphql::{Context, Object};
use entity::entities::pets;
use service::mutations::pet::PetMutationService;
use tracing::instrument;

#[derive(Default)]
pub struct PetMutation;

#[Object]
impl PetMutation {
    #[instrument(skip(self, input, ctx))]
    pub async fn add_pet(&self, ctx: &Context<'_>, input: NewPetInput) -> Result<Pet> {
        let db = ctx.data::<Database>()?;
        let conn = db.get_connection();
        let pet = PetMutationService::add_pet(conn, pets::ActiveModel::from(input)).await?;

        Ok(Pet::from(pet))
    }
}
