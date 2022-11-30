use std::path::{PathBuf, Path};

use rocket::{fs::NamedFile, tokio::io};

#[get("/help/<file..>?<command>")]
pub async fn help(command: Option<&str>, mut file: PathBuf) -> io::Result<NamedFile> {
    let page_directory_path = format!("{}/../help/build", env!("CARGO_MANIFEST_DIR"));

    if file.as_os_str() == "" || command.is_some() {
        file = PathBuf::from("index.html");
    }

    NamedFile::open(Path::new(&page_directory_path).join(file)).await
}

#[get("/admin/<file..>")]
pub async fn admin(mut file: PathBuf) -> io::Result<NamedFile> {
    let page_directory_path = format!("{}/../admin/build", env!("CARGO_MANIFEST_DIR"));

    if file.as_os_str() == "" || file.extension().is_none() {
        file = PathBuf::from("index.html");
    }

    NamedFile::open(Path::new(&page_directory_path).join(file)).await
}