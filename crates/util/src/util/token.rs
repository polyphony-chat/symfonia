// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use chorus::types::jwt::Claims;
use jsonwebtoken::TokenData;
use sqlx::PgPool;

use crate::{
	entities::User,
	errors::{Error, UserError},
};

pub async fn check_token(db: &PgPool, token: &str, jwt_secret: &str) -> Result<Claims, Error> {
	let decoding_key = jsonwebtoken::DecodingKey::from_base64_secret(jwt_secret).unwrap();
	let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
	validation.insecure_disable_signature_validation(); // TODO: Remove
	let token: TokenData<Claims> = jsonwebtoken::decode(token, &decoding_key, &validation).unwrap();

	let user = User::get_by_id(db, token.claims.id)
		.await
		.unwrap()
		.ok_or(Error::User(UserError::InvalidUser))?;

	if chrono::DateTime::from_timestamp(token.claims.iat, 0).unwrap() < user.data.valid_tokens_since
	{
		return Err(Error::User(UserError::InvalidToken));
	}

	// TODO: Check if user is banned or disabled

	Ok(token.claims)
}
