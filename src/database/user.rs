use crate::database::Queryer;
use polyphony_types::{entities::User, errors::Error, utils::Snowflake};

#[derive(Debug, Clone)]
pub struct UserService {}

impl UserService {
    pub async fn get_by_id<'c, C: Queryer<'c>>(
        db: C,
        id: &Snowflake,
    ) -> Result<Option<User>, Error> {
        sqlx::query_as("SELECT * FROM users WHERE id = ?")
            .bind(id)
            .fetch_optional(db)
            .await
            .map_err(Error::SQLX)
    }

    pub async fn find_by_user_and_discrim<'c, C: Queryer<'c>>(
        db: C,
        user: &str,
        discrim: &str,
    ) -> Result<Option<User>, Error> {
        sqlx::query_as("SELECT * FROM users WHERE username = ? AND discriminator = ?")
            .bind(user)
            .bind(discrim)
            .fetch_optional(db)
            .await
            .map_err(Error::SQLX)
    }

    pub async fn get_user_by_email_or_phone<'c, C: Queryer<'c>>(
        db: C,
        email: &str,
        phone: &str,
    ) -> Result<Option<User>, Error> {
        sqlx::query_as("SELECT * FROM users WHERE email = ? OR phone = ? LIMIT 1")
            .bind(email)
            .bind(phone)
            .fetch_optional(db)
            .await
            .map_err(Error::SQLX)
    }
}
