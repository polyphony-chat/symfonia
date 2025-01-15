/*
 *  This Source Code Form is subject to the terms of the Mozilla Public
 *  License, v. 2.0. If a copy of the MPL was not distributed with this
 *  file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chorus::types::{jwt, APIError, AuthError, LoginSchema};
use poem::{
    handler,
    web::{Data, Json},
    IntoResponse, Request, Response,
};
use reqwest::StatusCode;
use serde_json::json;

use crate::database::entities::{Config, User};

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

    let Some(user) = User::get_user_by_email_or_phone(db, &payload.login, "")
        .await
        .unwrap()
    else {
        return Err(APIError::Auth(AuthError::InvalidLogin));
    };

    if user.data.hash.is_some() {
        let actual_hash = PasswordHash::parse(
            user.data.hash.as_ref().unwrap(),
            argon2::password_hash::Encoding::B64,
        )
        .map_err(|_| AuthError::InvalidLogin)?; // TODO This probably should not return InvalidLogin but we either need to change the function header or add a variant in chorus
        let salt = crate::database::get_password_salt()
            .await
            .map_err(|_| AuthError::InvalidLogin)?; // TODO as above ^
        let computed_hash = Argon2::default()
            .hash_password(payload.password.as_bytes(), &salt)
            .map_err(|_| AuthError::InvalidLogin)?; // TODO as above ^

        if actual_hash != computed_hash {
            return Err(APIError::Auth(AuthError::InvalidLogin));
        }
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

    //let user_settings = user.get_settings()

    Ok(Response::builder()
        .body(
            json!({
                "token": token,
                "settings": {}
            })
            .to_string(),
        )
        .into_response())
}
