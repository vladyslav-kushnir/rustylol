use std::collections::{hash_map::Entry, HashMap};

use rustbreak::{deser::Ron, FileDatabase};

use super::{Command, Storage, StorageError, Variation};

pub struct FileSystemStorage {
    db: FileDatabase<HashMap<String, Command>, Ron>,
}

impl FileSystemStorage {
    pub fn new() -> Self {
        let db =
            FileDatabase::<HashMap<String, Command>, Ron>::load_from_path_or_default("test.ron")
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

    // fn add_command_variation(
    //     &self,
    //     command_name: String,
    //     variation: String,
    // ) -> Result<(), StorageError> {
    //     let command = self.get_command(command_name);

    //     match command {
    //         Some(mut cmd) => {
    //             let variation = Variation::new(variation);

    //             match cmd.variations.entry(variation.args_count) {
    //                 Entry::Occupied(_) => Err(StorageError::InternalError(
    //                     "Variation with this amount of args already exists".to_string(),
    //                 )),
    //                 Entry::Vacant(v) => {
    //                     v.insert(variation);

    //                     // ToDo: I think this is not needed since we modified original command?
    //                     // self.db.write(|data| {
    //                     //     data.insert(command_name, cmd);
    //                     // });

    //                     self.db
    //                         .save()
    //                         .map_err(|err| StorageError::DatabaseError(err))?;

    //                     Ok(())
    //                 }
    //             }
    //         }
    //         None => Err(StorageError::InternalError(
    //             "Command doesn't exist".to_string(),
    //         )),
    //     }
    // }
}
