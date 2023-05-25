use crate::database::Queryer;
use polyphony_types::{entities::UserSettings, errors::Error, utils::Snowflake};

pub struct UserSettingsService;

impl UserSettingsService {
    pub async fn get_by_index<'c, C: Queryer<'c>>(
        db: C,
        index: u64,
    ) -> Result<UserSettings, Error> {
        sqlx::query_as("SELECT * FROM user_settings WHERE index = ? limiq")
            .bind(index)
            .fetch_one(db)
            .await
            .map_err(Error::SQLX)
    }
}
