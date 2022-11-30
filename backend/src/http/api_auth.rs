use std::sync::Arc;

use rocket::{State, http::{Status, CookieJar}};

use crate::auth::AuthManager;

#[get("/api/auth/<provider>")]
pub fn get_auth(auth_manager: &State<Arc<AuthManager>>, provider: &str) -> Result<String, Status> {
    Ok(auth_manager.get_provider(provider).map_err(Status::from)?.get_url(provider))
}

#[get("/api/auth_callback?<state>&<code>")]
pub async fn auth_callback(auth_manager: &State<Arc<AuthManager>>, jar: &CookieJar<'_>, code: &str, state: &str) -> Result<String, Status> {
    let provider = auth_manager.get_provider(state);
    
    let access_token = provider.map_err(Status::from)?.get_access_token(code, state).await;

    if let Err(error) = &access_token {
        info!("access_token err: {}", error);
    }

    access_token.map_err(|_| Status::Forbidden)
}