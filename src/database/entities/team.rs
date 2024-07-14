use std::ops::{Deref, DerefMut};

use chorus::types::Snowflake;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

use crate::{database::entities::User, errors::Error};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Team {
    #[serde(flatten)]
    #[sqlx(flatten)]
    inner: chorus::types::Team,
}

impl Deref for Team {
    type Target = chorus::types::Team;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Team {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Team {
    pub async fn get_by_id(db: &MySqlPool, id: &Snowflake) -> Result<Option<Self>, Error> {
        sqlx::query_as("SELECT * FROM teams WHERE id = ?")
            .bind(id)
            .fetch_optional(db)
            .await
            .map_err(Error::SQLX)
    }

    // Helper functions

    pub async fn get_members(&self, db: &MySqlPool) -> Result<Vec<TeamMember>, Error> {
        TeamMember::get_by_team(db, self.id).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TeamMember {
    #[serde(flatten)]
    #[sqlx(flatten)]
    inner: chorus::types::TeamMember,
    pub user_id: Snowflake,
}

impl Deref for TeamMember {
    type Target = chorus::types::TeamMember;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for TeamMember {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl TeamMember {
    pub async fn get_by_id(db: &MySqlPool, id: Snowflake) -> Result<Option<Self>, Error> {
        sqlx::query_as("SELECT * FROM team_members WHERE id = ?")
            .bind(id)
            .fetch_optional(db)
            .await
            .map_err(Error::SQLX)
    }

    pub async fn get_by_team(db: &MySqlPool, team_id: Snowflake) -> Result<Vec<Self>, Error> {
        sqlx::query_as("SELECT * FROM team_members WHERE team_id = ?")
            .bind(team_id)
            .fetch_all(db)
            .await
            .map_err(Error::SQLX)
    }

    pub async fn get_by_user(db: &MySqlPool, user_id: Snowflake) -> Result<Vec<Self>, Error> {
        sqlx::query_as("SELECT * FROM team_members WHERE user_id = ?")
            .bind(user_id)
            .fetch_all(db)
            .await
            .map_err(Error::SQLX)
    }

    // Helper functions

    pub async fn get_team(&self, db: &MySqlPool) -> Result<Option<Team>, Error> {
        Team::get_by_id(db, &self.team_id).await
    }

    pub async fn get_user(&self, db: &MySqlPool) -> Result<Option<User>, Error> {
        User::get_by_id(db, self.user_id).await
    }
}
