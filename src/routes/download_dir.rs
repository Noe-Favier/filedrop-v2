use std::{fs::File, path::Path};
use rocket::response::content;
use rocket::response::status::NotFound;
use std::env;
use tempfile::NamedTempFile;

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
    let tempfile = NamedTempFile::new().expect("can't create empty temp file");

    zip_dir(path_to_dir, tempfile.as_file()).expect("can't zip");
    return Ok(content::RawMsgPack(Some(tempfile.reopen().expect("can't reopen the temp file"))));
}