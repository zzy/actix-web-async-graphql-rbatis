use async_graphql::Context;
use rbatis::rbatis::Rbatis;

use crate::users::{self, models::User};

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    // Get all Users,
    async fn all_users(
        &self,
        ctx: &Context<'_>,
    ) -> std::result::Result<Vec<User>, async_graphql::Error> {
        let my_pool = ctx.data_unchecked::<Rbatis>();
        users::services::all_users(my_pool).await
    }
}
