use std::error::Error as StdError;

use chorus::types::{APIError, AuthError};
use poem::error::ResponseError;
use poem::http::StatusCode;
use poem::Response;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    User(#[from] UserError),

    #[error(transparent)]
    Guild(#[from] GuildError),

    #[error(transparent)]
    Channel(#[from] ChannelError),

    #[error(transparent)]
    Invite(#[from] InviteError),

    #[error("SQLX error: {0}")]
    SQLX(#[from] sqlx::Error),

    #[error("Migration error: {0}")]
    SQLXMigration(#[from] sqlx::migrate::MigrateError),

    #[error("serde: {0}")]
    Serde(#[from] serde_json::Error),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    Chorus(#[from] chorus::types::APIError),

    #[error(transparent)]
    Rand(#[from] rand::Error),

    #[error(transparent)]
    Utf8(#[from] std::string::FromUtf8Error),
}

#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("EMAIL_INVALID")]
    InvalidEmail,
    #[error("DISCRIMINATOR_INVALID")]
    InvalidDiscriminator,
    #[error("INVALID_USER")]
    InvalidUser,
    #[error("INVALID_TOKEN")]
    InvalidToken,
    #[error("ALREADY_EXISTS")]
    AlreadyExists,
}

#[derive(Debug, thiserror::Error)]
pub enum GuildError {
    #[error("GUILD_NOT_FOUND")]
    InvalidGuild,
    #[error("MEMBER_NOT_FOUND")]
    MemberNotFound,
    #[error("ALREADY_IN_GUILD")]
    AlreadyInGuild,
}

#[derive(Debug, thiserror::Error)]
pub enum ChannelError {
    #[error("Unknown Channel")]
    InvalidChannel, // code 10003
}

#[derive(Debug, thiserror::Error)]
pub enum InviteError {
    #[error("INVALID_INVITE")]
    InvalidInvite,
}

impl ResponseError for Error {
    fn status(&self) -> StatusCode {
        match self {
            Error::User(err) => match err {
                UserError::InvalidEmail => StatusCode::BAD_REQUEST,
                UserError::InvalidDiscriminator => StatusCode::BAD_REQUEST,
                UserError::InvalidUser => StatusCode::NOT_FOUND,
                UserError::InvalidToken => StatusCode::UNAUTHORIZED,
                UserError::AlreadyExists => StatusCode::BAD_REQUEST,
            },
            Error::Guild(err) => match err {
                GuildError::InvalidGuild => StatusCode::NOT_FOUND,
                GuildError::MemberNotFound => StatusCode::NOT_FOUND,
                GuildError::AlreadyInGuild => StatusCode::BAD_REQUEST,
            },
            Error::Channel(err) => match err {
                ChannelError::InvalidChannel => StatusCode::NOT_FOUND,
            },
            Error::Invite(err) => match err {
                InviteError::InvalidInvite => StatusCode::NOT_FOUND,
            },
            Error::SQLX(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::SQLXMigration(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Serde(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::IO(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Chorus(err) => match err {
                APIError::Auth(auth_err) => match auth_err {
                    AuthError::InvalidLogin => StatusCode::UNAUTHORIZED,
                    AuthError::InvalidCaptcha => StatusCode::BAD_REQUEST,
                },
            },
            Error::Rand(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Utf8(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn as_response(&self) -> Response
    where
        Self: StdError + Send + Sync + 'static,
    {
        Response::builder()
            .status(self.status())
            .body(self.to_string())
    }
}
