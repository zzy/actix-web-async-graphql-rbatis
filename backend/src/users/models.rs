use serde::{Serialize, Deserialize};

#[rbatis::crud_enable]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub cred: String,
}

#[async_graphql::Object]
impl User {
    pub async fn id(&self) -> i32 {
        self.id
    }

    pub async fn email(&self) -> &str {
        self.email.as_str()
    }

    pub async fn username(&self) -> &str {
        self.username.as_str()
    }
}
