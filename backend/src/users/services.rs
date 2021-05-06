use async_graphql::{Error, ErrorExtensions};
use rbatis::rbatis::Rbatis;
use rbatis::crud::CRUD;

use crate::util::constant::GqlResult;
use crate::users::models::{NewUser, User};

// 查询所有用户
pub async fn all_users(my_pool: &Rbatis) -> GqlResult<Vec<User>> {
    let users = my_pool.fetch_list::<User>("").await.unwrap();

    if users.len() > 0 {
        Ok(users)
    } else {
        Err(Error::new("1-all-users")
            .extend_with(|_, e| e.set("details", "No records")))
    }
}

// 通过 email 获取用户
pub async fn get_user_by_email(
    my_pool: &Rbatis,
    email: &str,
) -> GqlResult<User> {
    let email_wrapper = my_pool.new_wrapper().eq("email", email);
    let user = my_pool.fetch_by_wrapper::<User>("", &email_wrapper).await;

    if user.is_ok() {
        Ok(user.unwrap())
    } else {
        Err(Error::new("email 不存在")
            .extend_with(|_, e| e.set("details", "1_EMAIL_NOT_EXIStS")))
    }
}

// 插入新用户
pub async fn new_user(
    my_pool: &Rbatis,
    mut new_user: NewUser,
) -> GqlResult<User> {
    new_user.email = new_user.email.to_lowercase();

    if self::get_user_by_email(my_pool, &new_user.email).await.is_ok() {
        Err(Error::new("email 已存在")
            .extend_with(|_, e| e.set("details", "1_EMAIL_EXIStS")))
    } else {
        new_user.cred =
            "P38V7+1Q5sjuKvaZEXnXQqI9SiY6ZMisB8QfUOP91Ao=".to_string();
        my_pool.save("", &new_user).await.expect("插入 user 数据时出错");

        self::get_user_by_email(my_pool, &new_user.email).await
    }
}
