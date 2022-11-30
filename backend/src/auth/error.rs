use rocket::http::Status;

#[derive(Debug)]
pub enum Error {
    MissingAuthHeader,
    InvalidAuthHeader,
    InvalidAuthProvider,
    AuthProviderError(super::auth_provider::Error),
}

impl From<Error> for Status {
    fn from(e: Error) -> Self {
        match e {
            Error::InvalidAuthHeader => Status::Forbidden,
            Error::InvalidAuthProvider => Status::Forbidden,
            Error::MissingAuthHeader => Status::Forbidden,
            Error::AuthProviderError(_) => Status::InternalServerError,
        }
    }
}

impl From<super::auth_provider::Error> for Error {
    fn from(e: super::auth_provider::Error) -> Self {
        Error::AuthProviderError(e)
    }
}