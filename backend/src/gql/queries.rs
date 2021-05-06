use async_graphql::Context;
use rbatis::rbatis::Rbatis;

use crate::util::constant::GqlResult;
use crate::users::{self, models::User};
pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    // 获取所有用户
    async fn all_users(&self, ctx: &Context<'_>) -> GqlResult<Vec<User>> {
        let my_pool = ctx.data_unchecked::<Rbatis>();
        users::services::all_users(my_pool).await
    }

    //根据 email 获取用户
    async fn get_user_by_email(
        &self,
        ctx: &Context<'_>,
        email: String,
    ) -> GqlResult<User> {
        let my_pool = ctx.data_unchecked::<Rbatis>();
        users::services::get_user_by_email(my_pool, &email).await
    }
}
