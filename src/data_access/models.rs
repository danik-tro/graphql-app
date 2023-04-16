use super::schema::accounts;
use diesel::Insertable;

#[derive(Insertable)]
#[diesel(table_name = accounts)]
pub struct NewAccountDao {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
}
