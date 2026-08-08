use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use cornucopia::deadpool_postgres;
use cornucopia::tokio_postgres;

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Authentication error")]
    AuthError,

    #[error("Environment variable error: {0}")]
    VarError(#[from] std::env::VarError),

    #[error("Database error: {0}")]
    Database(#[from] tokio_postgres::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Crypto error: {0}")]
    Crypto(#[from] chacha20poly1305::Error),

    #[error("Pool error: {0}")]
    Pool(#[from] deadpool_postgres::PoolError),

    #[error("Unhandled error")]
    Unhandled,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("{}", self);
        (StatusCode::INTERNAL_SERVER_ERROR, "-1").into_response()
    }
}

impl From<base64::DecodeError> for AppError {
    fn from(_err: base64::DecodeError) -> Self {
        Self::Unhandled
    }
}

impl From<std::string::FromUtf8Error> for AppError {
    fn from(_err: std::string::FromUtf8Error) -> Self {
        Self::Unhandled
    }
}

impl From<deadpool_postgres::CreatePoolError> for AppError {
    fn from(_err: deadpool_postgres::CreatePoolError) -> Self {
        Self::Unhandled
    }
}

impl From<bpaf::ParseFailure> for AppError {
    fn from(_err: bpaf::ParseFailure) -> Self {
        Self::Unhandled
    }
}
