use crate::{database::Queryer, errors::Error};
use poem::EndpointExt;
use std::ops::{Deref, DerefMut};

pub struct UserSettings(chorus::types::UserSettings);

impl Deref for UserSettings {
    type Target = chorus::types::UserSettings;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for UserSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl UserSettings {
    pub async fn get_by_index<'c, C: Queryer<'c>>(
        db: C,
        index: u64,
    ) -> Result<UserSettings, Error> {
        sqlx::query_as("SELECT * FROM user_settings WHERE index = ? limiq")
            .bind(index)
            .fetch_one(db)
            .await
            .map(Self)
            .map_err(Error::SQLX)
    }
}
