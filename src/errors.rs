#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    User(#[from] UserError),

    #[error("SQLX error: {0}")]
    SQLX(#[from] sqlx::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("EMAIL_INVALID")]
    InvalidEmail,
    #[error("DISCRIMINATOR_INVALID")]
    InvalidDiscriminator,
}
