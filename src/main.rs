use graphql_dataloaders::app::build_router;
use graphql_dataloaders::settings::Settings;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(build_router(Settings::new()?).await.unwrap().into_make_service())
        .await
        .unwrap();

    Ok(())
}
