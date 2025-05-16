// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use std::str::FromStr;

use email_address::EmailAddress;

use crate::errors::{Error, UserError};

pub fn adjust_email(email: &str) -> Result<EmailAddress, Error> {
	if email.is_empty() {
		return Err(Error::User(UserError::InvalidEmail));
	}

	// TODO: check accounts with uncommon email domains
	// TODO: replace .dots and +alternatives -> Gmail Dot Trick https://support.google.com/mail/answer/7436150 and https://generator.email/blog/gmail-generator
	EmailAddress::from_str(email).map_err(|e| Error::User(UserError::InvalidEmail))
}
