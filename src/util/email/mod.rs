use regex::Regex;

use crate::errors::{Error, UserError};

lazy_static::lazy_static! {
    static ref EMAIL_REGEX: Regex = Regex::new(r#"^(([^<>()[\]\\.,;:\s@"]+(\.[^<>()[\]\\.,;:\s@"]+)*)|(".+"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$"#).unwrap();
}

pub fn adjust_email(email: &str) -> Result<String, Error> {
    if email.is_empty() {
        return Err(Error::User(UserError::InvalidEmail));
    }

    if !EMAIL_REGEX.is_match(email) {
        return Err(Error::User(UserError::InvalidEmail));
    }

    // TODO: check accounts with uncommon email domains
    // TODO: replace .dots and +alternatives -> Gmail Dot Trick https://support.google.com/mail/answer/7436150 and https://generator.email/blog/gmail-generator
    return Ok(email.to_string());
}
