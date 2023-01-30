use std::collections::{hash_map::Entry, HashMap};

use rustbreak::{deser::Ron, FileDatabase};

use crate::settings;

use super::{Command, Storage, StorageError, Variation};

pub struct FileSystemStorage {
    db: FileDatabase<HashMap<String, Command>, Ron>,
}

impl FileSystemStorage {
    pub fn new(config: &settings::FileStorageConfig) -> Self {
        let db =
            FileDatabase::<HashMap<String, Command>, Ron>::load_from_path_or_default(config.path.to_owned())
                .expect("Database with commands should be created");

        db.load().expect("Database needs to be loaded");

        Self { db }
    }
}

impl Storage for FileSystemStorage {
    fn upsert_command(&self, command: Command) -> Result<(), StorageError> {
        self.db
            .write(|db| {
                db.insert(command.name.clone(), command);
            })
            .map_err(|err| StorageError::DatabaseError(err))?;

        self.db
            .save()
            .map_err(|err| StorageError::DatabaseError(err))
    }

    fn get_commands(&self) -> Option<Vec<Command>> {
        self.db
            .read(|data| data.values().cloned().collect::<Vec<Command>>())
            .ok()
    }

    fn get_command(&self, name: String) -> Option<Command> {
        self.db
            .read(|data| data.get(&name).map(|val| val.to_owned()))
            .ok()
            .flatten()
    }

    fn delete_command(&self, name: String) -> Result<(), StorageError> {
        self.db
            .write(|db| {
                db.remove(&name);
            })
            .map_err(|err| StorageError::DatabaseError(err))?;

        self.db
            .save()
            .map_err(|err| StorageError::DatabaseError(err))
    }
}
