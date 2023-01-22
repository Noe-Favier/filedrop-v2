use diesel::sqlite::SqliteConnection as SQLConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> SQLConnection {
    dotenv().ok(); //load .env vars
    
    let database_url = env::var("DATABASE_URL") //set db url form .env
    .expect("DATABASE_URL must be set"); //if none is set, crash
    
    SQLConnection::establish(&database_url) //connect to db
    .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}