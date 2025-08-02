use async_graphql::MergedObject;
use user::UserMutation;
mod user;
#[derive(MergedObject, Default)]
pub struct Mutation(UserMutation);
