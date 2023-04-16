pub mod resolvers;
pub mod types;

use async_graphql::{EmptySubscription, Schema, SimpleObject};
use resolvers::accounts::AccountsQuery;
use resolvers::auth::AuthMutations;

#[derive(SimpleObject, Default)]
pub struct Query {
    pub accounts: AccountsQuery,
}

#[derive(SimpleObject, Default)]
pub struct Mutation {
    sign: AuthMutations,
}

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;
