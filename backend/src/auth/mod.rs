use std::{sync::Arc, collections::HashMap};

use crate::settings::Auth;

pub mod error;
mod auth_value;
mod auth_provider;

pub use auth_value::*;
pub use auth_provider::*;

use self::providers::{google_oauth::GoogleOAuthClient, azure_oauth::AzureOAuthClient};

pub mod rocket_guard;
pub mod providers;

#[derive(PartialEq, Eq, Hash)]
pub enum AuthProviderKind {
    Google,
    Azure
}

impl TryFrom<&str> for AuthProviderKind {
    type Error = error::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "google" => Ok(AuthProviderKind::Google),
            "azure" => Ok(AuthProviderKind::Azure),
            _ => Err(error::Error::InvalidAuthProvider)
        }
    }
}

pub struct AuthManager {
    providers: HashMap<AuthProviderKind, Arc<Box<dyn AuthProvider>>>,
}

impl AuthManager {
    pub fn new(settings: &Auth) -> Self {
        let Auth { redirect_url, .. } = settings; 

        let providers = settings.providers.iter().map::<Result<_, error::Error>, _>(|provider| {
            let kind = AuthProviderKind::try_from(provider.name.as_str())?;

            let client: Arc<Box<dyn AuthProvider>> = match kind {
                AuthProviderKind::Google => Ok(Arc::<Box<dyn AuthProvider>>::new(Box::new(GoogleOAuthClient::new(provider.client_id.clone(), provider.client_secret.clone(), redirect_url.clone())?))),
                AuthProviderKind::Azure => Ok(Arc::<Box<dyn AuthProvider>>::new(Box::new(AzureOAuthClient::new(provider.client_id.clone(), provider.client_secret.clone(), redirect_url.clone(), provider.tenant_id.clone())?))),
                _ => Err(error::Error::InvalidAuthProvider)
            }?;

            Ok((kind, client))
        })
        .filter(|provider| provider.is_ok())
        .map(|provider| provider.unwrap())
        .collect();

        Self {
            providers
        }
    }

    pub fn get_provider(&self, kind: &str) -> Result<Arc<Box<dyn AuthProvider>>, error::Error> {
        let provider_kind = AuthProviderKind::try_from(kind)?;

        self.providers.get(&provider_kind).ok_or(error::Error::InvalidAuthProvider).map(|provider| provider.clone())
    }

    pub async fn get_identity(&self, header: &AuthHeader) -> Result<AuthValue, error::Error> {
        // Valid if any provider can recognize this AuthToken
        for provider in self.providers.values() {
            if let Ok(auth_value) = provider.get_identity(header).await {
                return Ok(auth_value);
            }
        }

        Err(error::Error::InvalidAuthHeader)
    }
}