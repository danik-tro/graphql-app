use super::models::NewAccountDao;
use crate::graphql::types::input_types::AccountInput;

impl From<(uuid::Uuid, &AccountInput)> for NewAccountDao {
    fn from((id, dto): (uuid::Uuid, &AccountInput)) -> Self {
        Self {
            id: id,
            username: dto.username.clone(),
            email: dto.email.clone(),
            first_name: dto.first_name.clone(),
            last_name: dto.last_name.clone(),
            password: dto.password.clone(),
        }
    }
}
