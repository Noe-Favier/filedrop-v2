use std::{time::SystemTime, path::PathBuf};
use chrono::prelude::{DateTime, Local};

extern crate serde;

#[derive(Clone, serde::Serialize)]
pub struct FileDropFile{
    pub name: String,
    pub ftype: String,

    pub size: u64,
    pub date_last_modified: String,

    pub locked: bool,
    pub pass_hash: String,

    pub full_path: PathBuf,
}

impl FileDropFile {

    //
    pub fn new(
        name: String,
        ftype: String,
        size:u64,
        dlm: SystemTime,
        full_path: PathBuf,
    ) -> FileDropFile {
        let dt: DateTime<Local> = dlm.clone().into();

        return FileDropFile{
            name:name,
            ftype: ftype,
            size: size,
            date_last_modified: format!("{}", dt.format("%v - %T")),
            locked: false,
            pass_hash: String::from(""),

            full_path: full_path,
        };
    }

    pub fn new_locked(
        name: String,
        ftype: String,
        size:u64,
        dlm: SystemTime,
        full_path: PathBuf,

        pass_hash: String,
    ) -> FileDropFile {
        let dt: DateTime<Local> = dlm.clone().into();

        return FileDropFile{
            name:name,
            ftype: ftype,
            size: size,
            date_last_modified: format!("{}", dt.format("%v - %T")),
            locked: true,
            pass_hash: pass_hash,

            full_path: full_path,
        };
    }
}