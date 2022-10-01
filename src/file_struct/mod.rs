use std::time::SystemTime;
use chrono::prelude::{DateTime, Local};

extern crate serde;

#[derive(Clone, serde::Serialize)]
pub struct FileDropFile{
    name: String,
    ftype: String,

    size: u64,
    date_last_modified: String,
}

impl FileDropFile {

    pub fn new(
        name: String,
        ftype: String,
        size:u64,
        dlm: SystemTime,
    ) -> FileDropFile {
        let dt: DateTime<Local> = dlm.clone().into();

        return FileDropFile{
            name:name,
            ftype: ftype,
            size: size,
            date_last_modified: format!("{}", dt.format("%v - %T")),
        };
    }
}