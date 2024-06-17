pub use channel::*;
pub use config::*;
pub use emoji::*;
pub use guild::*;
pub use invite::*;
pub use member::*;
pub use message::*;
pub use read_state::*;
pub use role::*;
pub use user::*;
pub use user_settings::*;
pub use webhook::*;

mod application;
mod attachment;
mod channel;
mod config;
mod emoji;
mod guild;
mod invite;
mod member;
mod message;
mod read_state;
mod role;
mod sticker;
mod template;
mod user;
mod user_settings;
mod voice_state;
mod webhook;

/*#[async_trait]
pub trait BaseObject<'a>: Sized {
    const TABLE: &'a str;
    type UpdateSchema: ?Sized;

    async fn get_by_id<'c, C: Queryer<'c>>(db: C, id: &Snowflake) -> Result<Option<Self>, Error> {
        sqlx::query_as(format!("SELECT * FROM {} WHERE id = ?", BaseObject::TABLE).as_str())
            .bind(id)
            .fetch_optional(db)
            .await
            .map(|res| res.map(Self::from))
            .map_err(Error::SQLX)
    }
}
TODO: Figure this out at some point to cut down on re-written code
*/
