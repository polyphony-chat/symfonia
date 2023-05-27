use crate::{database::Queryer, errors::Error};
use chorus::types::{Snowflake, UserData};
use futures::FutureExt;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    #[sqlx(flatten)]
    inner: chorus::types::User,
    pub data: sqlx::types::Json<UserData>,
    pub deleted: bool,
}

impl Deref for User {
    type Target = chorus::types::User;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for User {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl User {
    pub async fn create<'c, C: Queryer<'c>>(db: C) -> Result<Self, Error> {}

    pub async fn get_by_id<'c, C: Queryer<'c>>(
        db: C,
        id: &Snowflake,
    ) -> Result<Option<Self>, Error> {
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
    ) -> Result<Option<Self>, Error> {
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
    ) -> Result<Option<Self>, Error> {
        sqlx::query_as("SELECT * FROM users WHERE email = ? OR phone = ? LIMIT 1")
            .bind(email)
            .bind(phone)
            .fetch_optional(db)
            .await
            .map_err(Error::SQLX)
    }
}
