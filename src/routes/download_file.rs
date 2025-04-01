use rocket::response::content;
use std::{env, fs::File};

#[get("/<name>")]
pub fn download(name: &str) -> Result<content::RawMsgPack<Option<File>>, std::io::Error> {
    let files_path: String = env::var("files_path").unwrap_or(String::from("./files"));
    let filename: String = format!("{files}/{name}", files = files_path, name = name);
    Ok(content::RawMsgPack(File::open(&filename).ok()))
}