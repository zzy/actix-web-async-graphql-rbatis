use async_graphql::Context;
use rbatis::rbatis::Rbatis;

use crate::util::constant::GqlResult;
use crate::users::{self, models::User};
pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    // Get all Users
    async fn all_users(&self, ctx: &Context<'_>) -> GqlResult<Vec<User>> {
        let my_pool = ctx.data_unchecked::<Rbatis>();
        users::services::all_users(my_pool).await
    }
}
