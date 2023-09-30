use crate::{
    database::{
        entities::{Config, UserSettings},
        Queryer,
    },
    errors::Error,
};
use chorus::types::{Snowflake, UserData};
use chrono::Utc;
use futures::FutureExt;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::{
    default::Default,
    ops::{Deref, DerefMut},
};

#[derive(Debug, Clone, Default, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    #[sqlx(flatten)]
    inner: chorus::types::User,
    pub data: sqlx::types::Json<UserData>,
    pub deleted: bool,
    pub fingerprints: String, // TODO: Simple-array, should actually be a vec
    #[sqlx(rename = "settingsIndex")]
    pub settings_index: i32,
    pub rights: String,
    #[sqlx(skip)]
    pub settings: UserSettings,
    pub extended_settings: sqlx::types::Json<Value>,
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
    pub async fn create<'c, C: Queryer<'c> + Copy>(
        db: C,
        cfg: &Config,
        username: &str,
        password: Option<String>,
        email: Option<String>,
        fingerprint: Option<String>,
        date_of_birth: Option<String>,
        bot: bool,
    ) -> Result<Self, Error> {
        // TODO: trim username
        // TODO: generate discrim

        // TODO: dynamically figure out locale
        let user_settings = UserSettings::create(db, "en-US").await?;

        let password = if let Some(password) = password {
            Some(bcrypt::hash(password, 14).unwrap())
        } else {
            None
        };

        let user = Self {
            inner: chorus::types::User {
                username: username.to_string(),
                discriminator: "0001".to_string(),
                email: email.clone(),
                premium: cfg.defaults.user.premium,
                premium_type: cfg.defaults.user.premium_type,
                bot: Some(bot),
                verified: if cfg.defaults.user.verified {
                    Some(true)
                } else {
                    Some(false)
                },
                ..Default::default()
            },
            data: sqlx::types::Json(UserData {
                hash: password,
                valid_tokens_since: Utc::now(),
            }),
            fingerprints: if let Some(fingerprint) = fingerprint {
                fingerprint
            } else {
                String::default()
            },
            rights: cfg.register.default_rights.clone(),
            settings_index: user_settings.index as i32, // TODO: Idk why spacebar uses an int, it should be an unsigned bigint (u64)
            extended_settings: sqlx::types::Json(Value::Object(Map::default())),
            settings: user_settings,
            ..Default::default()
        };

        sqlx::query("INSERT INTO users (id, username, email, data, fingerprint, settingsIndex) VALUES (?, ?, ?, ?, ?, ?)")
            .bind(&user.id)
            .bind(username)
            .bind(email)
            .bind(&user.data)
            .bind(&user.fingerprints)
            .bind(user.settings.index)
            .execute(db)
            .await?;

        Ok(user)
    }

    async fn find_unused_discriminator<'c, C: Queryer<'c>>(
        db: C,
        cfg: &Config,
    ) -> Result<String, Error> {
        // TODO: intelligently find unused discriminator: https://dba.stackexchange.com/questions/48594/find-numbers-not-used-in-a-column
        todo!()
    }

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
