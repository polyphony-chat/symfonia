/*
 *  This Source Code Form is subject to the terms of the Mozilla Public
 *  License, v. 2.0. If a copy of the MPL was not distributed with this
 *  file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};
use sqlx::AnyPool;

use crate::errors::Error;

#[derive(Debug, Clone, Default, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserSettings {
    #[sqlx(flatten)]
    #[serde(flatten)]
    inner: chorus::types::UserSettings,
    pub index: u64,
}

impl Deref for UserSettings {
    type Target = chorus::types::UserSettings;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for UserSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl UserSettings {
    pub fn consume(inner: chorus::types::UserSettings, index: u64) -> Self {
        Self { inner, index }
    }

    pub async fn create(db: &AnyPool, locale: &str) -> Result<Self, Error> {
        let mut settings = Self {
            inner: chorus::types::UserSettings {
                locale: locale.to_string(),
                ..Default::default()
            },
            index: 0,
        };

        let res = sqlx::query("INSERT INTO user_settings (locale) VALUES (?)")
            .bind(locale)
            .execute(db)
            .await?;

        settings.index = res.last_insert_id();

        Ok(settings)
    }

    pub async fn get_by_index(db: &AnyPool, index: u64) -> Result<UserSettings, Error> {
        sqlx::query_as("SELECT * FROM user_settings WHERE index = ?")
            .bind(index)
            .fetch_one(db)
            .await
            .map_err(Error::SQLX)
    }
}
