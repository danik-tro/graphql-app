use async_graphql::{Context, Object, Result, SimpleObject};

#[derive(Default)]
pub struct AccountsQuery;

#[Object]
impl AccountsQuery {
    async fn account_info(&self, _ctx: &Context<'_>) -> Result<&str> {
        Ok("Account info")
    }
}
