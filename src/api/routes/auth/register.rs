/* 
 *  This Source Code Form is subject to the terms of the Mozilla Public
 *  License, v. 2.0. If a copy of the MPL was not distributed with this
 *  file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use chorus::types::jwt::generate_token;
use chorus::types::{APIError, AuthError, RegisterSchema};
use poem::web::{Data, Json};
use poem::{handler, IntoResponse, Request};
use serde_json::json;

use crate::database::entities::{Config, User};

#[handler]
pub async fn register(
    Data(db): Data<&sqlx::MySqlPool>,
    Data(cfg): Data<&Config>,
    Json(payload): Json<RegisterSchema>,
    req: &Request,
) -> Result<impl IntoResponse, APIError> {
    // TODO: Check reg token

    if !payload.consent {
        // TODO: Fail consent
    }

    // TODO: Check registration disabled
    if let Some(exists) =
        User::get_user_by_email_or_phone(db, payload.email.as_ref().unwrap().as_str(), "")
            .await
            .expect("Failed to check if user exists")
    {
        return Err(APIError::Auth(AuthError::InvalidLogin)); // TODO: Change error
    }

    // TODO: All field checks

    let user = User::create(
        db,
        cfg,
        &payload.username,
        payload.password,
        payload.email,
        payload.fingerprint,
        payload.date_of_birth,
        false,
    )
    .await
    .expect("Failed to create user");

    // TODO: Invite

    let token = generate_token(
        &user.id,
        user.email.clone().unwrap(),
        &cfg.security.jwt_secret,
    );

    Ok(Json(json!({"token": token})))
}
