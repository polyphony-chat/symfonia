use crate::database::UserService;
use poem::{
    handler,
    web::{Data, Json},
    IntoResponse, Request, Response,
};
use polyphony_types::{
    config::ConfigValue,
    schema::{APIError, AuthError, LoginSchema},
    utils::jwt,
};
use reqwest::StatusCode;
use serde_json::json;

#[handler]
pub async fn login(
    Data(db): Data<&sqlx::MySqlPool>,
    Data(cfg): Data<&ConfigValue>,
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

    let Some(user) = UserService::get_user_by_email_or_phone(db, &payload.login, "").await? else {
        return Err(APIError::Auth(AuthError::InvalidLogin));
    };

    if !user
        .data
        .hash
        .as_ref()
        .map(|hash| bcrypt::verify(&payload.password, hash).unwrap_or_default())
        .unwrap_or_default()
    {
        return Err(APIError::Auth(AuthError::InvalidLogin));
    }

    if cfg.login.require_verification && !user.verified {
        return Err(APIError::Auth(AuthError::Unverified));
    }

    // TODO: MFA / WebauthN

    if payload.undelete.unwrap_or(false) {
        if user.disabled {
            todo!()
        }
        if user.deleted {
            todo!()
        }
    } else {
        if user.disabled {
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
        user.email.unwrap_or_default(),
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
