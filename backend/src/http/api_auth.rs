use std::sync::Arc;

use rocket::{State, http::{Status, CookieJar}, serde::json::Json};
use serde::Serialize;

use crate::auth::{AuthManager};

#[derive(Serialize)]
pub struct AccessToken {
    pub token: String,
    pub expires_in: Option<u64>,
}

impl From<crate::auth::AccessToken> for AccessToken {
    fn from(value: crate::auth::AccessToken) -> Self {
        Self {
            token: value.token,
            expires_in: value.expires_in.and_then(|v| Some(v.as_secs()))
        }
    }
}

#[get("/api/auth/<provider>/url")]
pub fn get_auth_redirect_url(auth_manager: &State<Arc<AuthManager>>, provider: &str) -> Result<String, Status> {
    Ok(auth_manager.get_provider(provider).map_err(Status::from)?.get_url(provider))
}

#[get("/api/auth_callback?<state>&<code>")]
pub async fn auth_callback(auth_manager: &State<Arc<AuthManager>>, _jar: &CookieJar<'_>, code: &str, state: &str) -> Result<Json<AccessToken>, Status> {
    let provider = auth_manager.get_provider(state);
    
    let access_token = provider.map_err(Status::from)?.get_access_token(code, state).await.map_err(|_| Status::Forbidden)?;

    Ok(Json(access_token.into()))
}