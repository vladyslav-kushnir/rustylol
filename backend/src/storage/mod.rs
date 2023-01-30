mod file_system;
mod s3_storage;

use std::collections::HashMap;

pub use file_system::FileSystemStorage;
pub use s3_storage::S3Storage;

use rocket::serde::{Deserialize, Serialize};
use rustbreak::RustbreakError;

use strfmt::strfmt;

use crate::settings;

#[derive(Debug)]
pub enum StorageError {
    InternalError(String),
    DatabaseError(RustbreakError),
}

pub trait Storage: Send + Sync {
    fn get_commands(&self) -> Option<Vec<Command>>;

    fn get_command(&self, name: String) -> Option<Command>;

    fn upsert_command(&self, command: Command) -> Result<(), StorageError>;

    fn delete_command(&self, name: String) -> Result<(), StorageError>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Command {
    pub name: String,
    pub variations: HashMap<usize, Variation>,
}

impl Command {
    pub fn new(name: String, variations: Vec<Variation>) -> Self {
        Self {
            name,
            variations: variations
                .into_iter()
                .map(|v| (v.args_count, v))
                .collect::<HashMap<_, _>>(),
        }
    }

    pub fn get_redirect_url(&self, args: Vec<&str>) -> Option<String> {
        let handlebars_args = args
            .iter()
            .enumerate()
            .map(|(idx, arg)| (format!("{}", idx), arg))
            .collect::<HashMap<_, _>>();

        match self.variations.get(&args.len()) {
            Some(variation) => match strfmt(variation.url_pattern.as_str(), &handlebars_args) {
                Ok(result) => Some(result),
                Err(_) => None,
            },
            None => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Variation {
    pub args_count: usize,
    pub url_pattern: String,
}

impl Variation {
    pub fn new(url_pattern: String) -> Self {
        // ToDo: Do some better logic here to make validation?
        let args_count = url_pattern.matches("{").count();

        Self {
            args_count,
            url_pattern,
        }
    }
}

pub fn initialize_storage(settings: &settings::Storage) -> Box<dyn Storage> {
    match settings {
        settings::Storage::File { config } => Box::new(FileSystemStorage::new(config)),
        settings::Storage::S3 { config } => Box::new(S3Storage::new(config)),
    }
}
