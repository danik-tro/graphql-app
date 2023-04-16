use super::errors::DataAccessError;
use super::models::NewAccountDao;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};

pub async fn create_account(
    connection: &mut AsyncPgConnection,
    dao: NewAccountDao,
) -> Result<(), DataAccessError> {
    use crate::data_access::schema::accounts::dsl;
    diesel::insert_into(dsl::accounts)
        .values(&dao)
        .execute(connection)
        .await?;
    Ok(())
}
