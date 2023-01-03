use rocket::response::content;
use std::{env, fs::File};

#[get("/<name>")]
pub fn download(name: &str) -> content::RawMsgPack<Option<File>> {
    let files_path: String = env::var("files_path").unwrap_or(String::from("./files"));
    let filename: String = format!("{files}/{name}", files = files_path, name = name);
    return content::RawMsgPack(File::open(&filename).ok())
}