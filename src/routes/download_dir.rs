use std::{fs::File, path::Path};
use rocket::{response::content};
use rocket::response::status::NotFound;
use std::env;
use tempfile::tempfile;

use crate::utils::dir_zipper::zip_dir;

#[get("/<name>/<_formatted_name>")]
pub fn download_dir(name: &str, _formatted_name: &str) -> Result<content::RawMsgPack<Option<File>>, NotFound<String>> {
    let files_path: String = env::var("files_path").unwrap_or(String::from("./files"));
    let dirname: String = format!("{dirs}/{name}", dirs = files_path, name = name);
    let path_to_dir = Path::new(&dirname).to_path_buf();

    if !path_to_dir.is_dir() {
        return Err(NotFound("Should be a dir".to_string()));
    }

    //**************** TEMP FILE CREATION ****************//
    let tempfile = tempfile().expect("can't create empty temp file");

    println!("added file to {:?}", &tempfile);

    zip_dir(path_to_dir, &tempfile).expect("can't zip");
    return Ok(content::RawMsgPack(Some(tempfile)));
}