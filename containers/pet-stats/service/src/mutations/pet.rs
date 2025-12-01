use entity::entities::pets;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DbConn, DbErr, DeleteResult, EntityTrait};
use tracing::{debug, error, info, instrument};

use crate::utils::{commit_transaction, get_current_time, start_transaction};

pub struct PetMutationService;

impl PetMutationService {
    #[instrument(skip(db))]
    pub async fn add_pet(db: &DbConn, pet: pets::ActiveModel) -> Result<pets::Model, DbErr> {
        let txn = start_transaction(db).await?;
        let new_pet = pet.insert(&txn).await?;
        commit_transaction(txn).await?;
        Ok(new_pet)
    }

    #[instrument(skip(db))]
    pub async fn remove_pet(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        // FIXME: Remove exec with returning till fixing bug on 1.1.7 sea_orm
        // pets::Entity::delete(pets::ActiveModel {
        //     id: Set(id),
        //     ..Default::default()
        // })
        // .exec_with_returning(db)
        // .await
        // .inspect_err(|e| error!("{:?}", e))?
        // .inspect(|p| info!("Removed successfully pet: {:?}", p.id))
        // .ok_or_else(|| DbErr::RecordNotFound("Pet Not Found".to_owned()))
        //
        //
        //

        let remove_target = pets::ActiveModel {
            id: Set(id),
            ..Default::default()
        };

        pets::Entity::delete(remove_target)
            .exec(db)
            .await
            .inspect(|dr| debug!("row_affected - {:?}", dr.rows_affected))
            .inspect_err(|e| error!("{:?}", e))
    }

    #[instrument(skip(db))]
    pub async fn update_pet(db: &DbConn, mut pet: pets::ActiveModel) -> Result<pets::Model, DbErr> {
        let txn = start_transaction(db).await?;
        let now = get_current_time();
        pet.updated_at = Set(now);
        let pet = pet.update(&txn).await?;
        commit_transaction(txn).await?;
        Ok(pet)
    }
}
