use oauth2::{basic::BasicClient, ClientId, ClientSecret, AuthUrl, TokenUrl, RedirectUrl, CsrfToken, Scope, AuthorizationCode, TokenResponse};
use oauth2::reqwest::async_http_client;

use crate::auth::auth_provider::AccessToken;
use crate::auth::{self, AuthHeader, AuthValue, auth_provider::{AuthProvider, Error}};

pub struct GoogleOAuthClient {
    client: BasicClient
}

impl GoogleOAuthClient {
    pub fn new(client_id: String, client_secret: String, redirect_url: String) -> Result<Self, Error> {
        Ok(Self {
            client: BasicClient::new(
                ClientId::new(client_id),
                Some(ClientSecret::new(client_secret)),
                AuthUrl::new(String::from("https://accounts.google.com/o/oauth2/v2/auth"))?,
                Some(TokenUrl::new(String::from("https://www.googleapis.com/oauth2/v4/token"))?),
            ).set_redirect_uri(RedirectUrl::new(redirect_url)?)
        })
    }
}

#[rocket::async_trait]
impl AuthProvider for GoogleOAuthClient {
    fn get_url(&self, state: &str) -> String {
        let (authorize_url, _csrf_state) = self.client.authorize_url(|| CsrfToken::new(state.to_string()))
        .add_scope(Scope::new(
            "openid".to_string(),
        ))
        .url();

        authorize_url.to_string()
    }

    async fn get_access_token(&self, code: &str, state: &str) -> Result<AccessToken, auth::auth_provider::Error> {
        let token = self.client.exchange_code(AuthorizationCode::new(code.to_string())).request_async(async_http_client).await?;

        Ok(AccessToken { token: token.access_token().secret().clone(), expires_in: token.expires_in() })
    }

    async fn get_identity(&self, header: &AuthHeader) -> Result<AuthValue, auth::auth_provider::Error> {
        info!("Checking Google Auth");

        match header {
            AuthHeader::Bearer { token } => {
                let url = format!("https://www.googleapis.com/oauth2/v1/tokeninfo?access_token={}", token);

                let response =  reqwest::get(&url).map_err(|e| auth::auth_provider::Error::Internal(e.to_string()))?;

                // ToDo: Does google have a concept of roles?

                match response.status().is_success() {
                    true => Ok(AuthValue::Bearer { token: token.clone(), roles: vec![] }),
                    false => Err(auth::auth_provider::Error::InvalidAuth("Google couldn't confirm token authenticity".to_string()))
                }
            },
            _ => Err(auth::auth_provider::Error::InvalidAuth("Google Auth - Wrong Header".to_string()))
        }
    }
}