#[macro_use]
extern crate rocket;
extern crate strfmt;

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use rocket::{
    fs::NamedFile,
    futures::io,
    http::Status,
    response::{status::NoContent, Redirect},
    serde::json::Json,
    State,
};
use storage::Variation;

use crate::{
    models::AddCommandRequest,
    storage::{Command, FileSystemStorage, Storage},
};

mod models;
mod storage;

#[get("/<command>")]
fn index(storage: &State<Box<dyn Storage>>, command: String) -> Result<Redirect, Status> {
    let command_parts = command.split(" ").collect::<Vec<&str>>();

    let command = storage.get_command(command_parts[0].to_string());

    match command {
        Some(c) => {
            let args = command_parts.into_iter().skip(1).collect::<Vec<_>>();
            if args.len() == 1 && args[0] == "--help" {
                return Ok(Redirect::to(format!("/help?command={}", c.name)));
            }

            let redirect_uri = c.get_redirect_url(args);

            println!("{:?}", redirect_uri);

            match redirect_uri {
                Some(uri) => Ok(Redirect::to(uri)),
                None => Err(Status::NoContent),
            }
        }
        None => Err(Status::NotFound),
    }
}

#[get("/help/<file..>?<command>")]
async fn help_static_files(command: Option<&str>, mut file: PathBuf) -> io::Result<NamedFile> {
    let page_directory_path = format!("{}/../help/build", env!("CARGO_MANIFEST_DIR"));

    if file.as_os_str() == "" || command.is_some() {
        file = PathBuf::from("index.html");
    }

    NamedFile::open(Path::new(&page_directory_path).join(file)).await
}

#[get("/admin/<file..>")]
async fn admin_static_files(mut file: PathBuf) -> io::Result<NamedFile> {
    let page_directory_path = format!("{}/../admin/build", env!("CARGO_MANIFEST_DIR"));

    if file.as_os_str() == "" {
        file = PathBuf::from("index.html");
    }

    NamedFile::open(Path::new(&page_directory_path).join(file)).await
}

#[post("/api/command", data = "<request>")]
fn upsert_command(
    storage: &State<Box<dyn Storage>>,
    request: Json<AddCommandRequest>,
) -> Result<NoContent, Status> {
    let data = request.0;

    let command = Command::new(
        data.name,
        data.variations
            .into_iter()
            .map(|v| Variation::new(v))
            .collect::<Vec<_>>(),
    );

    storage
        .upsert_command(command)
        .map_err(|_| Status::InternalServerError)?;

    Ok(NoContent)
}

// #[post("/api/command/<name>", data = "<request>")]
// fn add_variation(
//     storage: &State<Box<dyn Storage>>,
//     name: String,
//     request: Json<AddCommandRequest>,
// ) -> Result<NoContent, Status> {
//     let data = request.0;

//     let command = Command::new(
//         data.name,
//         data.variations
//             .into_iter()
//             .map(|v| Variation::new(v))
//             .collect::<Vec<_>>(),
//     );

//     storage
//         .add_command(command)
//         .map_err(|_| Status::InternalServerError)?;

//     Ok(NoContent)
// }

#[get("/api/commands")]
fn get_commands(storage: &State<Box<dyn Storage>>) -> Json<Vec<Command>> {
    Json(storage.get_commands().unwrap_or_default())
}

#[launch]
fn rocket() -> _ {
    let storage = FileSystemStorage::new();

    storage
        .upsert_command(Command::new(
            "test".to_string(),
            vec![
                Variation::new("{0}".to_string()),
                Variation::new("{0}/{1}".to_string()),
            ],
        ))
        .expect("test command");

    rocket::build()
        .mount(
            "/",
            routes![
                index,
                admin_static_files,
                help_static_files,
                upsert_command,
                get_commands
            ],
        )
        .manage(Box::new(storage) as Box<dyn Storage>)
}
