use thiserror::Error;

/**
First lets creat an Internal(String) variant of errors we dont want or
cannot handle gracefully
*/
#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Internal Error")]
    Internal(String),
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    InvalidArgument(String),
}

impl warp::reject::Reject for Error {}

impl std::convert::From<sqlx::migrate::MigrateError> for Error {
    fn from(err: sqlx::migrate::MigrateError) -> Self {
        Error::Internal(err.to_string())
    }
}

impl std::convert::From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error::InvalidArgument(err.to_string())
    }
}

impl std::convert::From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => Error::NotFound("Not Found".into()),
            _ => Error::Internal(err.to_string()),
        }
    }
}
