use std::path::{Path, PathBuf};

use rocket::{routes, Build, Rocket, fs::NamedFile, get};

#[rocket::launch]
fn rocket() -> Rocket<Build> {
    let figment = rocket::Config::figment()
        .merge(("address", "0.0.0.0"))
        .merge(("port", 80))
        .merge(("limits.json", 32768))
        .merge(("log_level", "debug"));

    rocket::custom(figment)
        .mount(
            "/",
            routes![
                index,
                static_files
            ],
        )
}

#[get("/")]
pub async fn index() -> Option<NamedFile> {
    let path = Path::new("static").join("index.html");
    NamedFile::open(path).await.ok()
}

#[get("/<path..>")]
pub async fn static_files(path: PathBuf) -> Option<NamedFile> {
    let path = Path::new("static").join(path);
    NamedFile::open(path).await.ok()
}