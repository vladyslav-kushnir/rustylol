use rocket::{response::status::NoContent, State, http::Status, serde::json::Json};

use crate::{storage::{Variation, Command, Storage}, auth::AuthValue, models::AddCommandRequest};

#[get("/api/commands")]
pub fn get_commands(storage: &State<Box<dyn Storage>>, auth: AuthValue) -> Json<Vec<Command>> {
    Json(storage.get_commands().unwrap_or_default())
}

#[post("/api/command", data = "<request>")]
pub fn upsert_command(
    storage: &State<Box<dyn Storage>>,
    request: Json<AddCommandRequest>,
    auth: AuthValue,
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

#[delete("/api/command/<name>")]
pub fn delete_command(
    storage: &State<Box<dyn Storage>>,
    name: String,
    auth: AuthValue,
) -> Result<NoContent, Status> {
    storage
        .delete_command(name)
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