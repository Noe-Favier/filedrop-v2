use std::{fs::{read_dir, DirEntry}, path::Path, time::SystemTime};
use rocket_dyn_templates::{context, Template};
use rocket::response::content;
use std::env;

use crate::model::{dir_struct::FileDropDir, file_struct::FileDropFile};

#[get("/")]
pub fn index() -> content::RawHtml<Template> {
    
    let mut file_list: Vec<FileDropFile> = Vec::new();
    let mut dir_list: Vec<FileDropDir> = Vec::new();

    for f in read_dir(Path::new(
        &env::var("files_path").unwrap_or(String::from("./files")),
    ))
    .unwrap()
    {
        let entry: DirEntry = f.unwrap();

            if entry.file_type().unwrap().is_dir() && &env::var("enable_folder").unwrap_or(String::from("false")) == "true" {
                //we got a folder
                let directory: DirEntry = entry;
                let filename: String = (directory.file_name().to_str().ok_or("/invalid filename/")).unwrap().to_string();
                let date_lm: SystemTime = directory.metadata().unwrap().modified().unwrap();

                dir_list.push(FileDropDir::new(
                    filename.to_owned(),
                    date_lm.to_owned(),
                    directory.path()
                ));


            } else if entry.file_type().unwrap().is_file() {
                //we got a file
                let file: DirEntry = entry;
                let filename: String = (file.file_name().to_str().ok_or("/invalid filename/")).unwrap().to_string();
                let filetype: String = mime_guess::from_path(&filename).first_or_octet_stream().to_string();
                let filesize: u64    = file.metadata().unwrap().len();
                let date_lm: SystemTime = file.metadata().unwrap().modified().unwrap();

                
                file_list.push(FileDropFile::new(
                    filename.to_owned(),
                    filetype.to_owned(),
                    filesize.to_owned(),
                    date_lm.to_owned()
                ));
        }
        
    }

    content::RawHtml(Template::render(
        "index",
        context! { index: "active", about:"inactive", files:file_list, dirs:dir_list },
    ))
}