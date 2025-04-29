// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub use audit_log::*;
pub use channel::*;
pub use config::*;
pub use emoji::*;
pub use guild::*;
pub use guild_template::*;
pub use invite::*;
pub use member::*;
pub use message::*;
pub use note::*;
pub use read_state::*;
pub use recipient::*;
pub use relationship::*;
pub use role::*;
pub use sticker::*;
pub use user::*;
pub use user_settings::*;
pub use voice_state::*;
pub use webhook::*;

use crate::SharedEventPublisher;

mod application;
mod attachment;
mod audit_log;
mod channel;
mod config;
mod emoji;
mod guild;
mod guild_template;
mod invite;
mod member;
mod message;
mod note;
mod read_state;
mod recipient;
mod relationship;
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
			.map_err(Error::Sqlx)
	}
}
TODO: Figure this out at some point to cut down on re-written code
*/
