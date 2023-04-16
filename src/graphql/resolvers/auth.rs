use crate::data_access::{repository, PgConnectionPool};
use crate::graphql::types::input_types::AccountInput;
use async_graphql::{Context, Object, Result};

#[derive(Default)]
pub struct AuthMutations;

#[Object]
impl AuthMutations {
    #[graphql(name = "up")]
    async fn sign_up(&self, ctx: &Context<'_>, input: AccountInput) -> Result<&str> {
        let mut connection = ctx
            .data_unchecked::<PgConnectionPool>()
            .as_ref()
            .get()
            .await?;

        repository::create_account(&mut connection, (uuid7::new_v7(), &input).into()).await?;
        Ok("Signup mutation")
    }

    #[graphql(name = "in")]
    async fn sign_in(&self, _ctx: &Context<'_>) -> Result<&str> {
        Ok("Sign in mutation")
    }
}
