use entity::entities::{pets, pets::Model as Pet};
use sea_orm::{ColumnTrait, DbConn, DbErr, EntityTrait, QueryFilter};
use tracing::{error, info, instrument};

pub struct PetQuery;

impl PetQuery {
    #[instrument(skip(db))]
    pub async fn get_pets_by_user_id(db: &DbConn, user_id: i32) -> Result<Vec<Pet>, DbErr> {
        pets::Entity::find()
            .filter(pets::Column::UserId.eq(user_id))
            .all(db)
            .await
            .inspect(|ps| info!("Found user: {:?} pets count: {:?}", user_id, ps.len()))
            .inspect_err(|e| error!("Error occur: {:?}", e))
    }

    #[instrument(skip(db))]
    pub async fn get_pet_by_id(db: &DbConn, pet_id: i32) -> Result<Pet, DbErr> {
        pets::Entity::find_by_id(pet_id)
            .one(db)
            .await
            .inspect(|p| {
                if let Some(opt_pet) = p {
                    info!("Found pet: {:?}", opt_pet.id);
                } else {
                    info!("No pet found for id: {}", pet_id);
                }
            })
            .inspect_err(|e| error!("Error occur: {:?}", e))?
            .ok_or_else(|| DbErr::RecordNotFound("Pet Not Found".to_owned()))
    }
}
