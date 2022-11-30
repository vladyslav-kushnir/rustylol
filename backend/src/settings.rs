use std::fmt;

use config::{Config, ConfigError, File};
use serde::Deserialize;

const CONFIG_FILE_PATH: &str = "./config/default.json";
const CONFIG_FILE_PREFIX: &str = "./config/";

#[derive(Debug, Deserialize, Clone)]
pub struct AuthProvider {
    pub name: String,
    pub client_id: String,
    pub client_secret: String,
    pub tenant_id: Option<String>
}

#[derive(Debug, Deserialize, Clone)]
pub struct Auth {
    pub enabled: bool,
    pub providers: Vec<AuthProvider>,
    pub redirect_url: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ENV {
    Development,
    Testing,
    Production,
}

impl fmt::Display for ENV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ENV::Development => write!(f, "development"),
            ENV::Testing => write!(f, "testing"),
            ENV::Production => write!(f, "production"),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub auth: Auth,
    pub env: ENV,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let env = std::env::var("RUN_ENV").unwrap_or_else(|_| "development".into());
        let mut s = Config::new();
        s.set("env", env.as_str())?;

        s.merge(File::with_name(CONFIG_FILE_PATH))?;
        s.merge(File::with_name(&format!("{}{}", CONFIG_FILE_PREFIX, env)).required(false))?;

        s.try_into()
    }
}