use async_graphql::MergedObject;
use user::UserQuery;

use pet::PetQuery;

mod pet;
mod user;
#[derive(MergedObject, Default)]
pub struct Query(UserQuery, PetQuery);
