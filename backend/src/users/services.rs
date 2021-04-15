use async_graphql::{Error, ErrorExtensions};
use rbatis::rbatis::Rbatis;
use rbatis::crud::CRUD;

use crate::util::constant::GqlResult;
use crate::users::models::User;

pub async fn all_users(my_pool: &Rbatis) -> GqlResult<Vec<User>> {
    let users = my_pool.fetch_list::<User>("").await.unwrap();

    if users.len() > 0 {
        Ok(users)
    } else {
        Err(Error::new("1-all-users")
            .extend_with(|_, e| e.set("details", "No records")))
    }
}
