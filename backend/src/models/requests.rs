use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AddCommandRequest {
    pub name: String,
    pub variations: Vec<String>,
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(crate = "rocket::serde")]
// pub struct AddCommandVariationRequest {
//     pub name: String,
//     pub variations: Vec<String>,
// }
