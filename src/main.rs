extern crate mime_guess;
mod model;
mod utils;
mod routes;
mod handlers;
mod services;
mod middlewares;

use crate::middlewares::path_traversal_guard::PathTraversalGuard;
use crate::routes::files::file_routes;
use crate::services::env_service::{AppEnv, EnvService};
use poem::{EndpointExt, Route, Server};
use std::fs;
use std::fs::create_dir_all;
use std::string::String;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let env_service: &EnvService = EnvService::instance();

    // File root checkup \\
    let file_path: String = env_service.file_path();
    let allow_create: bool = env_service.allow_create();

    /// Check if the file path exists ; if not, create it if allowed
    if fs::metadata(&file_path).is_err() {
        if allow_create {
            //file_path isn't a folder or doesn't exists, try to create it :
            match create_dir_all(&file_path) {
                Ok(_) => println!("Created file path at {}", &file_path),
                Err(_error) => panic!("can't create folder")
            }

            assert!(fs::metadata(&file_path)?.is_dir()); //stop if it didn't worked
        } else {
            println!("Will not create {} because 'allow_create' is disabled in .env", &file_path);
            panic!("the directory provided does not exists and can't be created")
        }
    }

    //\\
    let app = Route::new()
        .nest("/api/files", file_routes())
        .data(env_service.app_config())
        .with(PathTraversalGuard)
        ;

    Server::new(
        TcpListener::bind(format!("{}:{}", env_service.app_host(), env_service.app_port()))
    )
        .run(app)
        .await
    //\\
}
