#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    User(#[from] UserError),

    #[error("SQLX error: {0}")]
    SQLX(#[from] sqlx::Error),

    #[error("serde: {0}")]
    Serde(#[from] serde_json::Error),

    #[error(transparent)]
    IO(#[from] std::io::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("EMAIL_INVALID")]
    InvalidEmail,
    #[error("DISCRIMINATOR_INVALID")]
    InvalidDiscriminator,
}
