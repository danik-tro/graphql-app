use async_graphql::{Context, Object, Result};

#[derive(Default)]
pub struct AuthMutations;

#[Object]
impl AuthMutations {
    #[graphql(name = "up")]
    async fn sign_up(&self, _ctx: &Context<'_>) -> Result<&str> {
        Ok("Signup mutation")
    }

    #[graphql(name = "in")]
    async fn sign_in(&self, _ctx: &Context<'_>) -> Result<&str> {
        Ok("Sign in mutation")
    }
}
