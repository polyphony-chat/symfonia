// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ops::{Deref, DerefMut};

use chorus::types::Snowflake;

#[derive(Debug, Clone)]
pub struct GuildTemplate {
	inner: chorus::types::GuildTemplate,
	pub id: Snowflake,
}

impl Deref for GuildTemplate {
	type Target = chorus::types::GuildTemplate;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl DerefMut for GuildTemplate {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}
