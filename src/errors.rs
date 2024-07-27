/*
 *  This Source Code Form is subject to the terms of the Mozilla Public
 *  License, v. 2.0. If a copy of the MPL was not distributed with this
 *  file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::error::Error as StdError;

use chorus::types::{APIError, AuthError, Rights};
use poem::{error::ResponseError, http::StatusCode, Response};

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

    #[error(transparent)]
    RateLimit(#[from] RateLimitError),

    #[error(transparent)]
    Reaction(#[from] ReactionError),

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

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    Tungstenite(#[from] tokio_tungstenite::tungstenite::Error),

    #[error(transparent)]
    Gateway(#[from] GatewayError),
}

#[derive(Debug, thiserror::Error)]
pub enum GatewayError {
    #[error("UNEXPECTED_MESSAGE")]
    UnexpectedMessage,
    #[error("TIMEOUT")]
    Timeout,
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
    #[error("MISSING_RIGHTS")]
    MissingRights(Rights),
}

#[derive(Debug, thiserror::Error)]
pub enum GuildError {
    #[error("GUILD_NOT_FOUND")]
    InvalidGuild,
    #[error("MEMBER_NOT_FOUND")]
    MemberNotFound,
    #[error("ALREADY_IN_GUILD")]
    AlreadyInGuild,
    #[error("ROLE_NOT_FOUND")]
    InvalidRole,
    #[error("UNKNOWN_BAN")]
    BanNotFound,
    #[error("ALREADY_BANNED")]
    BanAlreadyExists,
    #[error("EMOJI_NOT_FOUND")]
    InvalidEmoji,
    #[error("MAXIMUM_EMOJIS_REACHED({0})")]
    MaxEmojisReached(i32),
    #[error("MISSING_PERMISSIONS")] // TODO: Make this display the missing permission(s)
    InsufficientPermissions, /*(PermissionFlags)*/
    #[error("FEATURE_IS_MUTABLE")]
    FeatureIsImmutable,
    #[error("STICKER_NOT_FOUND")]
    StickerNotFound,
    #[error("MAXIMUM_ROLES_REACHED")]
    RoleLimitReached(u16),
    #[error("ROLE_NOT_FOUND")]
    RoleNotFound,
    #[error("TEMPLATE_NOT_FOUND")]
    TemplateNotFound,
    #[error("TEMPLATE_NO_SOURCE")]
    NoSourceGuild,
    #[error("UNKNOWN_VOICE_STATE")]
    VoiceStateNotFound,
}

#[derive(Debug, thiserror::Error)]
pub enum ChannelError {
    #[error("Unknown Channel")]
    InvalidChannel, // code 10003
    #[error("Invalid Channel Type")]
    InvalidChannelType,
    #[error("Message Content length over max character limit")]
    MessageTooLong,
    #[error("Empty messages are not allowed")]
    EmptyMessage,
    #[error("Invalid Message")]
    InvalidMessage,
    #[error("You cannot delete more than {0} messages")]
    TooManyMessages(u32),
    #[error("Maxmimum pins reached")]
    MaxPinsReached,
    #[error("Maxmimum webhooks reached")]
    MaxWebhooksReached,
    #[error("User is already a recipient of this channel")]
    InvalidRecipient,
}

#[derive(Debug, thiserror::Error)]
pub enum InviteError {
    #[error("INVALID_INVITE")]
    InvalidInvite,
}

#[derive(Debug, thiserror::Error)]
pub enum RateLimitError {
    #[error("TOO_MANY_MESSAGES")]
    TooManyMessages,
}

#[derive(Debug, thiserror::Error)]
pub enum ReactionError {
    #[error("INVALID_REACTION")]
    Invalid,
    #[error("REACTION_ALREADY_EXISTS")]
    AlreadyExists,
    #[error("REACTION_NOT_FOUND")]
    NotFound,
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
                UserError::MissingRights(_) => StatusCode::UNAUTHORIZED,
            },
            Error::Guild(err) => match err {
                GuildError::InvalidGuild => StatusCode::NOT_FOUND,
                GuildError::MemberNotFound => StatusCode::NOT_FOUND,
                GuildError::AlreadyInGuild => StatusCode::BAD_REQUEST,
                GuildError::InvalidRole => StatusCode::NOT_FOUND,
                GuildError::BanNotFound => StatusCode::NOT_FOUND,
                GuildError::BanAlreadyExists => StatusCode::BAD_REQUEST,
                GuildError::InvalidEmoji => StatusCode::NOT_FOUND,
                GuildError::MaxEmojisReached(_) => StatusCode::BAD_REQUEST,
                GuildError::InsufficientPermissions => StatusCode::UNAUTHORIZED,
                GuildError::FeatureIsImmutable => StatusCode::BAD_REQUEST,
                GuildError::StickerNotFound => StatusCode::NOT_FOUND,
                GuildError::RoleLimitReached(_) => StatusCode::BAD_REQUEST,
                GuildError::RoleNotFound => StatusCode::NOT_FOUND,
                GuildError::TemplateNotFound => StatusCode::NOT_FOUND,
                GuildError::NoSourceGuild => StatusCode::INTERNAL_SERVER_ERROR,
                GuildError::VoiceStateNotFound => StatusCode::NOT_FOUND,
            },
            Error::Channel(err) => match err {
                ChannelError::InvalidChannel => StatusCode::NOT_FOUND,
                ChannelError::InvalidChannelType => StatusCode::BAD_REQUEST,
                ChannelError::MessageTooLong => StatusCode::PAYLOAD_TOO_LARGE,
                ChannelError::EmptyMessage => StatusCode::BAD_REQUEST,
                ChannelError::InvalidMessage => StatusCode::NOT_FOUND,
                ChannelError::TooManyMessages(_) => StatusCode::BAD_REQUEST,
                ChannelError::MaxPinsReached => StatusCode::BAD_REQUEST,
                ChannelError::MaxWebhooksReached => StatusCode::BAD_REQUEST,
                ChannelError::InvalidRecipient => StatusCode::NOT_FOUND,
            },
            Error::Invite(err) => match err {
                InviteError::InvalidInvite => StatusCode::NOT_FOUND,
            },
            Error::RateLimit(err) => match err {
                RateLimitError::TooManyMessages => StatusCode::TOO_MANY_REQUESTS,
            },
            Error::Reaction(err) => match err {
                ReactionError::Invalid => StatusCode::NOT_FOUND,
                ReactionError::AlreadyExists => StatusCode::BAD_REQUEST,
                ReactionError::NotFound => StatusCode::NOT_FOUND,
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
            Error::Reqwest(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Tungstenite(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Gateway(err) => match err {
                GatewayError::UnexpectedMessage => StatusCode::BAD_REQUEST,
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
