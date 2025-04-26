// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ops::{Deref, DerefMut};

use chorus::types::Snowflake;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Attachment {
	#[sqlx(flatten)]
	inner: chorus::types::Attachment,
	pub message_id: Option<Snowflake>,
}

impl Deref for Attachment {
	type Target = chorus::types::Attachment;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl DerefMut for Attachment {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}
