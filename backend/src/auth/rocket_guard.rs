use std::sync::Arc;

use rocket::{request::FromRequest, http::Status, Request, outcome::Outcome};

use super::{AuthValue, error::Error, AuthManager, AuthHeader};

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthValue {
    type Error = Error;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, (Status, Self::Error), ()> {
        let auth_manager = match request.rocket().state::<Arc<AuthManager>>() {
            Some(s) => s,
            None => return Outcome::Failure((Status::Unauthorized, Error::MissingAuthHeader)) // ToDo: Probably another error?
        };

        let auth_header_raw = request.headers().get_one("Authorization");

        let auth_header = match AuthHeader::try_from(auth_header_raw) {
            Ok(value) => value,
            Err(err) => return Outcome::Failure((Status::Unauthorized, err))
        };

        match auth_manager.get_identity(&auth_header).await {
            Ok(auth_value) => Outcome::Success(auth_value),
            Err(_) => Outcome::Failure((Status::Unauthorized, Error::InvalidAuthHeader))
        }
    }
}