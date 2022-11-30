#[macro_use]
extern crate rocket;

#[macro_use]
extern crate lazy_static;

use std::{
    path::PathBuf, sync::Arc,
};

use auth::AuthManager;
use cors::CORS;
use rocket::{
    http::{Status},
    response::{status::{NoContent}, Redirect},
    State,
};

use crate::{
    storage::{FileSystemStorage, Storage},
};

mod models;
mod storage;
mod cors;
mod auth;
mod settings;
mod http;

lazy_static! {
    static ref CONFIG: settings::Settings =
        settings::Settings::new().expect("config can be loaded");
}

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

#[options("/api/<_anything..>")]
fn options(_anything: PathBuf) -> NoContent {
    NoContent
}

#[launch]
fn rocket() -> _ {
    let storage = FileSystemStorage::new();

    let auth_manager = Arc::new(AuthManager::new(&CONFIG.auth));

    rocket::build()
        .mount(
            "/",
            routes![
                index,
                options,
                http::static_files::admin,
                http::static_files::help,
                http::api_commands::upsert_command,
                http::api_commands::delete_command,
                http::api_commands::get_commands,
                http::api_auth::get_auth,
                http::api_auth::auth_callback
            ],
        )
        .attach(CORS)
        .manage(Box::new(storage) as Box<dyn Storage>)
        .manage(auth_manager)
}
