//! Error package

use thiserror::Error;

/// Custom error from the project.
#[derive(Debug, Error)]
pub enum Error {
    /// Error coming from reqwest calls.
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    /// Error coming from reqwest_middleware.
    #[error(transparent)]
    ReqwestMiddleware(#[from] reqwest_middleware::Error),
    /// Error coming from trying to follow a [NamedApiResource] with no URL.
    #[error("error while following empty url")]
    FollowEmptyURL,
}
