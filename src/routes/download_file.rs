use rocket::response::content;
use std::{env, fs::File};

use crate::model::user::User;

#[get("/<name>")]
pub fn download(name: &str, _user: User) -> content::RawMsgPack<Option<File>> {
    let files_path: String = env::var("files_path").unwrap_or(String::from("./files"));
    let filename: String = format!("{files}/{name}", files = files_path, name = name);
    return content::RawMsgPack(File::open(&filename).ok())
}