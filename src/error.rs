//! Error package

use thiserror::Error;

/// Custom error from the project.
#[derive(Debug, Error)]
pub enum Error {
    /// Error coming from reqwest calls.
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    /// Error coming from `reqwest_middleware`.
    #[error(transparent)]
    ReqwestMiddleware(#[from] reqwest_middleware::Error),
    /// Error raised when an Url can't be parsed.
    #[error("couldn't parse `{0:?}` to a valid url")]
    UrlParse(String),
    /// Error raised during [`RustemonClient`] build, when the [Url] doesn't end
    /// with a '/'.
    #[error("url `{0}` must end with a `/`")]
    NoTrailingSlash(String),
    /// Error coming from trying to follow an instance with no URL.
    #[error("error while following empty url")]
    FollowEmptyURL,
}
