use std::time::SystemTime;
use chrono::prelude::{DateTime, Local};

use crate::file_struct::{FileDropFile};

extern crate serde;

#[derive(Clone, serde::Serialize)]
pub struct FileDropDir{
    name: String,

    size: u64,
    date_last_modified: String,

    files: Vec<FileDropFile>
}

impl FileDropDir {

    pub fn new(
        name: String,
        size:u64,
        dlm: SystemTime,
        files: Vec<FileDropFile>
    ) -> FileDropDir {
        let dt: DateTime<Local> = dlm.clone().into();

        return FileDropDir{
            name:name,
            
            size: size,
            date_last_modified: format!("{}", dt.format("%v - %T")),
            
            files: files
        };
    }
}