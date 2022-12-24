use dotenv::dotenv;
use rocket::response::content;
use rocket_dyn_templates::{context, Template};
use std::env;
use std::string::String;
use std::time::{SystemTime};
use std::{fs, path::Path};
use std::fs::{File, create_dir_all, DirEntry};

mod file_struct;
mod dir_struct;

extern crate tera;

extern crate mime_guess;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_include_static_resources;

static_response_handler! {
    "/favicon.ico" => favicon => "favicon",
    "/folder-img.svg" => folder_img => "folder-img"
}

#[get("/")]
fn index() -> content::RawHtml<Template> {
    
    let mut file_list: Vec<file_struct::FileDropFile> = Vec::new();
    let mut dir_list: Vec<dir_struct::FileDropDir> = Vec::new();

    for f in fs::read_dir(Path::new(
        &env::var("files_path").unwrap_or(String::from("./files")),
    ))
    .unwrap()
    {
        let entry: DirEntry = f.unwrap();

            if entry.file_type().unwrap().is_dir() {
                //we got a folder
                let directory: DirEntry = entry;
                let filename: String = (directory.file_name().to_str().ok_or("/invalid filename/")).unwrap().to_string();
                let date_lm: SystemTime = directory.metadata().unwrap().modified().unwrap();

                dir_list.push(dir_struct::FileDropDir::new(
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

                
                file_list.push(file_struct::FileDropFile::new(
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

#[get("/<name>")]
fn download(name: &str) -> content::RawMsgPack<Option<File>> {
    let files_path: String = env::var("files_path").unwrap_or(String::from("./files"));
    let filename: String = format!("{files}/{name}", files = files_path, name = name);
    content::RawMsgPack(File::open(&filename).ok())
}

#[get("/about")]
fn about() -> content::RawHtml<Template> {
    content::RawHtml(Template::render(
        "about",
        context! { index: "inactive", about:"active" },
    ))
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    /* FILE_PATH CHECKUP */
    let file_path: String = env::var("files_path").unwrap_or(String::from("./files"));
    if fs::metadata(&file_path).is_err() {
        if env::var("allow_create").unwrap_or(String::from("true")) == String::from("true") {
            //FILE_PATH isn't a folder or doesn't exists, try to create it :
            match create_dir_all(&file_path) {
                Ok(_) => println!("Created file path at {file_path}", file_path = &file_path),
                Err(_error) => panic!("can't create folder")
            }

            assert!(fs::metadata(&file_path).unwrap().is_dir()); //stop if it didn't worked
            
        } else {
            println!("Will not create {file_path} because 'allow_create' is disabled in .env", file_path = &file_path);
            panic!("the directory provided does not exists and can't be created")
        }
    }
    //\\

    rocket::build()
        .attach(static_resources_initializer!(
            "favicon" => "assets/favicon.ico",
            "folder-img" => "assets/folder-img.svg"
        ))
        .mount("/", routes![index, about])
        .mount("/file", routes![download])
        .mount("/", routes![favicon, folder_img]) //assets
        .attach(Template::fairing())
}
