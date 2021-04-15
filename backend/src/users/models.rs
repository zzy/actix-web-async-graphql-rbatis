use serde::{Serialize, Deserialize};

#[rbatis::crud_enable]
#[derive(async_graphql::SimpleObject, Serialize, Deserialize, Clone, Debug)]
#[graphql(complex)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub cred: String,
}

#[async_graphql::ComplexObject]
impl User {
    pub async fn from(&self) -> String {
        let mut from = String::new();
        from.push_str(&self.username);
        from.push_str("<");
        from.push_str(&self.email);
        from.push_str(">");

        from
    }
}
