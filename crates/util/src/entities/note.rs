// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ops::{Deref, DerefMut};

use chorus::types::{Snowflake, UserNote};
use serde::{Deserialize, Serialize};

use crate::errors::Error;

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
/// A note that a user has written about another user. The target user may be
/// the author of the note.
pub struct Note {
	#[sqlx(flatten)]
	inner: UserNote,
}

impl Deref for Note {
	type Target = UserNote;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl DerefMut for Note {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

impl Note {
	pub fn into_inner(self) -> UserNote {
		self.inner
	}

	pub fn as_inner(&self) -> &UserNote {
		&self.inner
	}

	pub fn as_inner_mut(&mut self) -> &mut UserNote {
		&mut self.inner
	}

	/// Retrieve all notes from a user by their ID
	pub async fn get_by_author_id(
		author_id: Snowflake,
		db: &sqlx::PgPool,
	) -> Result<Vec<Self>, Error> {
		sqlx::query_as("SELECT * from notes WHERE author_id = $1")
			.bind(author_id)
			.fetch_all(db)
			.await
			.map_err(Error::Sqlx)
	}
}

// TODO: Move to symfonia again
// #[cfg(test)]
// mod note_unit_tests {
//     #[sqlx::test(fixtures(path = "../../../fixtures", scripts("notes")))]
//     async fn test_get_by_author_id(db: sqlx::PgPool) {
//         let notes = super::Note::get_by_author_id(7250861145186111490.into(),
// &db)             .await
//             .unwrap();
//         assert_eq!(notes.len(), 1);
//         let note = notes[0].as_inner();
//         assert_eq!(note.author_id, 7250861145186111490.into());
//         assert_eq!(note.target_id, 7250861145186111491.into());
//         assert_eq!(note.content, "This is a note");
//     }
// }
