use chorus::types::{APIError, AuthError};
use poem::error::ResponseError;
use poem::http::StatusCode;
use poem::Response;
use std::error::Error as StdError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    User(#[from] UserError),

    #[error(transparent)]
    Guild(#[from] GuildError),

    #[error(transparent)]
    Channel(#[from] ChannelError),

    #[error("SQLX error: {0}")]
    SQLX(#[from] sqlx::Error),

    #[error("serde: {0}")]
    Serde(#[from] serde_json::Error),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    Chorus(#[from] chorus::types::APIError),
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
            Error::SQLX(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Serde(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::IO(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Chorus(err) => match err {
                APIError::Auth(auth_err) => match auth_err {
                    AuthError::InvalidLogin => StatusCode::UNAUTHORIZED,
                    AuthError::InvalidCaptcha => StatusCode::BAD_REQUEST,
                },
            },
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
