// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use chorus::types::{APIError, AuthError, LoginSchema, jwt};
use poem::{
	IntoResponse, Request, Response, handler,
	web::{Data, Json},
};
use reqwest::StatusCode;
use serde_json::json;
use util::entities::{Config, User};

#[handler]
pub async fn login(
	Data(db): Data<&sqlx::PgPool>,
	Data(cfg): Data<&Config>,
	Json(payload): Json<LoginSchema>,
	req: &Request,
) -> Result<impl IntoResponse, APIError> {
	if cfg.login.require_captcha && cfg.security.captcha.enabled {
		if payload.captcha_key.is_none() {
			return Err(APIError::Auth(AuthError::InvalidCaptcha));
		}

		let ip = req.remote_addr().to_string();
		log::info!(target: "symfonia::auth", "Got client ip: {}", ip);

		// TODO: verify captcha
	}

	let Some(user) = User::get_user_by_email_or_phone(db, &payload.login, "").await.unwrap() else {
		return Err(APIError::Auth(AuthError::InvalidLogin));
	};

	if let Some(hash) = &user.data.hash {
		let password_hash = match PasswordHash::parse(hash, argon2::password_hash::Encoding::B64) {
			Ok(pw_hash) => pw_hash,
			Err(e) => {
				log::warn!("Couldn't parse hash for user id {}: {e}", user.id);
				return Err(AuthError::InvalidLogin.into());
			}
		};
		Argon2::default()
			.verify_password(payload.password.as_bytes(), &password_hash)
			.map_err(|_| AuthError::InvalidLogin)?;
	}

	if cfg.login.require_verification && !user.verified.unwrap_or_default() {
		return Err(APIError::Auth(AuthError::InvalidLogin));
	}

	// TODO: MFA / WebauthN

	if payload.undelete.unwrap_or(false) {
		if user.disabled.unwrap_or_default() {
			todo!()
		}
		if user.deleted {
			todo!()
		}
	} else {
		if user.disabled.unwrap_or_default() {
			return Ok(Response::builder()
				.status(StatusCode::BAD_REQUEST)
				.body(
					json!({
						"message": "auth:login.ACCOUNT_DISABLED",
						"code": 20013
					})
					.to_string(),
				)
				.into_response());
		}
		if user.deleted {
			return Ok(Response::builder()
				.status(StatusCode::BAD_REQUEST)
				.body(
					json!({
						"message": "This account is scheduled for deletion",
						"code": 20011
					})
					.to_string(),
				)
				.into_response());
		}
	}

	let token = jwt::generate_token(
		&user.id,
		user.email.clone().unwrap_or_default().as_str(),
		&cfg.security.jwt_secret,
	);

	Ok(Response::builder()
		.body(
			json!({
				"token": token,
				"settings": user.settings
			})
			.to_string(),
		)
		.into_response())
}
