use std::fmt::Display;

use oauth2::url::ParseError;
use serde::Serialize;

use super::{AuthHeader, AuthValue};

#[derive(Debug)]
pub enum Error {
    Internal(String),
    InvalidAuth(String)
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Internal(err) => write!(f, "{}", err),
            Self::InvalidAuth(err) => write!(f, "invalid auth: {}", err)
        }
    }
}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        Error::Internal(e.to_string())
    }
}

impl<RE, T> From<oauth2::RequestTokenError<RE, T>> for Error where
RE: std::error::Error + 'static,
T: oauth2::ErrorResponse + 'static, {
    fn from(e: oauth2::RequestTokenError<RE, T>) -> Self {
        Error::Internal(e.to_string())
    }
}

#[derive(Serialize)]
pub struct AccessToken {
    pub token: String,
    pub expires_in: Option<std::time::Duration>,
}

#[rocket::async_trait]
pub trait AuthProvider: Send + Sync {
    fn get_url(&self, state: &str) -> String;
    async fn get_access_token(&self, code: &str, state: &str) -> Result<AccessToken, Error>;
    async fn get_identity(&self, header: &AuthHeader) -> Result<AuthValue, Error>;
}