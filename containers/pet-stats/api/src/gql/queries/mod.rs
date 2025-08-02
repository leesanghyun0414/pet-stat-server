use async_graphql::MergedObject;
use user::UserQuery;

mod user;

#[derive(MergedObject, Default)]
pub struct Query(UserQuery);

