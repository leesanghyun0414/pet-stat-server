use async_graphql::MergedObject;
use user::UserMutation;

use crate::gql::mutations::pet::PetMutation;
mod pet;
mod user;
#[derive(MergedObject, Default)]
pub struct Mutation(UserMutation, PetMutation);
