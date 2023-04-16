use async_graphql::InputObject;

#[derive(Debug, InputObject)]
pub struct AccountInput {
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
}
