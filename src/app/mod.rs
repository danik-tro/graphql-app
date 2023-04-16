use crate::data_access::{build_connection_pool, DataAccessError};
use crate::graphql::{AppSchema, Mutation, Query};
use crate::settings::Settings;
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::{self, IntoResponse},
    routing::get,
    Router,
};

async fn graphql_handler(schema: Extension<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

pub async fn build_router(settings: Settings) -> Result<Router, AppError> {
    let pool = build_connection_pool(&settings).await?;

    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(settings)
        .data(pool)
        .finish();

    Ok(Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(Extension(schema)))
}

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("DataAccessError: {0}")]
    DataAccess(DataAccessError),
}

impl From<DataAccessError> for AppError {
    fn from(err: DataAccessError) -> Self {
        Self::DataAccess(err)
    }
}
