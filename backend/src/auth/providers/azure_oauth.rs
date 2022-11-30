use azure_jwt::AzureAuth;
use oauth2::{basic::BasicClient, ClientId, ClientSecret, AuthUrl, TokenUrl, RedirectUrl, CsrfToken, Scope, AuthorizationCode, TokenResponse};
use oauth2::reqwest::async_http_client;

use crate::auth::{self, AuthHeader, AuthValue, auth_provider::{AuthProvider, Error}};

pub struct AzureOAuthClient {
    client: BasicClient
}

impl AzureOAuthClient {
    pub fn new(client_id: String, client_secret: String, redirect_url: String, tenant_id: Option<String>) -> Result<Self, Error> {
        let tenant_id = tenant_id.unwrap_or("common".to_string());

        Ok(Self {
            client: BasicClient::new(
                ClientId::new(client_id),
                Some(ClientSecret::new(client_secret)),
                AuthUrl::new(String::from(format!("https://login.microsoftonline.com/{}/oauth2/v2.0/authorize", tenant_id)))?,
                Some(TokenUrl::new(format!("https://login.microsoftonline.com/{}/oauth2/v2.0/token", tenant_id))?),
            ).set_redirect_uri(RedirectUrl::new(redirect_url)?)
        })
    }
}

#[rocket::async_trait]
impl AuthProvider for AzureOAuthClient {
    fn get_url(&self, state: &str) -> String {
        let (authorize_url, _csrf_state) = self.client.authorize_url(|| CsrfToken::new(state.to_string()))
        .add_scope(Scope::new(
            "openid".to_string(),
        ))
        .url();

        authorize_url.to_string()
    }

    async fn get_access_token(&self, code: &str, state: &str) -> Result<String, auth::auth_provider::Error> {
        let token = self.client.exchange_code(AuthorizationCode::new(code.to_string())).request_async(async_http_client).await?;

        info!("{}", serde_json::to_string(&token).unwrap());

        Ok(token.access_token().secret().clone())
    }

    async fn get_identity(&self, header: &AuthHeader) -> Result<AuthValue, auth::auth_provider::Error> {
        match header {
            AuthHeader::Bearer { token } => {
                let client_id = self.client.client_id().to_string();
                let mut az_auth = AzureAuth::new(client_id).unwrap();

                // ToDo: roles?

                az_auth.validate_token(token).map_err(|e| auth::auth_provider::Error::InvalidAuth)?;

                Ok(AuthValue::Bearer { token: token.clone(), roles: vec![] })
            },
            _ => Err(auth::auth_provider::Error::InvalidAuth)
        }
    }
}