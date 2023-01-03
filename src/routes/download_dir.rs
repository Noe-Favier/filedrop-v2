use std::{fs::File, path::Path};
use rocket::response::content;
use std::env;

use crate::utils::dir_zipper::zip_dir;

#[get("/<name>/<_formatted_name>")]
pub fn download_dir(name: &str, _formatted_name: &str) -> content::RawMsgPack<Option<File>> {
    let files_path: String = env::var("files_path").unwrap_or(String::from("./files"));
    let dirname: String = format!("{dirs}/{name}", dirs = files_path, name = name);
    let path_to_dir = Path::new(&dirname).to_path_buf();
    let zipped_dir_path = zip_dir(path_to_dir).expect("can't zip");
    return content::RawMsgPack(File::open(&zipped_dir_path).ok());
}