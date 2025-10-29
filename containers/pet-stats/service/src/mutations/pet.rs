use entity::entities::pets;
use sea_orm::{ActiveModelTrait, DbConn, DbErr};
use tracing::instrument;

use crate::utils::{commit_transaction, start_transaction};

pub struct PetMutationService;

impl PetMutationService {
    #[instrument(skip(db))]
    pub async fn add_pet(db: &DbConn, pet: pets::ActiveModel) -> Result<pets::Model, DbErr> {
        let txn = start_transaction(db).await?;
        let new_pet = pet.insert(&txn).await?;
        commit_transaction(txn).await?;
        Ok(new_pet)
    }
}
