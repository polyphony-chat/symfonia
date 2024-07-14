use std::ops::{Deref, DerefMut};

use bitflags::Flags;
use chorus::types::{ApplicationFlags, Snowflake};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool, Row};

use crate::{
    database::entities::{Config, user::User},
    errors::{Error, UserError},
};

#[derive(Debug, Clone, Default, Serialize, Deserialize, sqlx::FromRow)]
pub struct Application {
    #[sqlx(flatten)]
    inner: chorus::types::Application,
    pub owner_id: Snowflake,
    pub bot_user_id: Option<Snowflake>,
    pub team_id: Option<Snowflake>,
}

impl Deref for Application {
    type Target = chorus::types::Application;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Application {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Application {
    pub async fn create(
        db: &MySqlPool,
        cfg: &Config,
        name: &str,
        description: &str,
        owner_id: Snowflake,
        verify_key: &str,
        flags: ApplicationFlags,
        create_bot_user: bool,
        redirect_uris: Vec<String>,
    ) -> Result<Self, Error> {
        let bot_user_id = if create_bot_user {
            let bot_user = User::create(db, cfg, name, None, None, None, None, true).await?;

            Some(bot_user.id.to_owned())
        } else {
            None
        };

        let application = Self {
            inner: chorus::types::Application {
                name: name.to_string(),
                summary: Some(description.to_string()),
                verify_key: verify_key.to_string(),
                flags,
                redirect_uris: Some(sqlx::types::Json(redirect_uris)),
                ..Default::default()
            },
            owner_id: owner_id.to_owned(),
            bot_user_id,
            team_id: None,
        };

        let _res = sqlx::query("INSERT INTO applications (id, name, description, hook, bot_public, verify_key, owner_id, flags, integration_public, discoverability_state, discovery_eligibility_flags, redirect_uris) VALUES (?, ?, ?, true, true, ?, ?, ?, true, 1, 2240, ?)")
            .bind(&application.id)
            .bind(name)
            .bind(description)
            .bind(verify_key)
            .bind(owner_id)
            .bind(flags.bits())
            .bind(&application.redirect_uris)
            .execute(db)
            .await?;

        Ok(application)
    }

    pub async fn get_by_id(db: &MySqlPool, id: Snowflake) -> Result<Option<Self>, Error> {
        sqlx::query_as("SELECT * FROM applications WHERE id = ?")
            .bind(id)
            .fetch_optional(db)
            .await
            .map_err(Error::SQLX)
    }

    pub async fn get_by_owner(db: &MySqlPool, owner_id: Snowflake) -> Result<Vec<Self>, Error> {
        sqlx::query_as("SELECT * FROM applications WHERE owner_id = ?")
            .bind(owner_id)
            .fetch_all(db)
            .await
            .map_err(Error::SQLX)
    }

    pub async fn get_by_team(db: &MySqlPool, team_id: Snowflake) -> Result<Vec<Self>, Error> {
        sqlx::query_as("SELECT * FROM applications WHERE team_id = ?")
            .bind(team_id)
            .fetch_all(db)
            .await
            .map_err(Error::SQLX)
    }

    pub async fn count_by_owner(db: &MySqlPool, owner_id: Snowflake) -> Result<i32, Error> {
        sqlx::query("SELECT COUNT(*) FROM applications WHERE owner_id = ?")
            .bind(owner_id)
            .fetch_one(db)
            .await
            .map_err(Error::SQLX)
            .map(|r| r.get::<i32, _>(0))
    }

    pub async fn count_by_team(db: &MySqlPool, team_id: Snowflake) -> Result<i32, Error> {
        sqlx::query("SELECT COUNT(*) FROM applications WHERE team_id = ?")
            .bind(team_id)
            .fetch_one(db)
            .await
            .map_err(Error::SQLX)
            .map(|r| r.get::<i32, _>(0))
    }

    pub async fn get_owner(&self, db: &MySqlPool) -> Result<User, Error> {
        let u = User::get_by_id(db, self.owner_id).await?.unwrap(); // Unwrap the option since this should absolutely never fail
        Ok(u)
    }

    pub async fn populate_relations(&mut self, db: &MySqlPool) -> Result<(), Error> {
        if let Some(bot_user_id) = self.bot_user_id {
            self.bot = User::get_by_id(db, bot_user_id)
                .await?
                .map(|user| user.to_inner());
        }

        self.owner = User::get_by_id(db, self.owner_id)
            .await?
            .ok_or(Error::User(UserError::InvalidUser))?
            .to_inner();
        Ok(())
    }

    pub fn public_json(&self) -> String {
        serde_json::to_string(&self.inner).unwrap()
    }
}
