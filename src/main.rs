mod model;
mod utils;
mod routes;

use dotenv::dotenv;
use rocket_dyn_templates::Template;
use std::env;
use std::string::String;
use std::fs;
use std::fs::create_dir_all;

use routes::{about::about, download_dir::download_dir, download_file::download, index::index};

extern crate tera;
extern crate mime_guess;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_include_static_resources;

static_response_handler! {
    "/favicon.ico" => favicon => "favicon",
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
        .mount("/dir", routes![download_dir])
        .mount("/", routes![favicon]) //assets
        .attach(Template::fairing())
}
