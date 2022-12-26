use chrono::prelude::{DateTime, Local};
use std::{
    fs::{read_dir},
    path::{PathBuf},
    time::SystemTime,
};

use crate::file_struct::FileDropFile;

extern crate serde;

#[derive(Clone, serde::Serialize)]
pub struct FileDropDir {
    pub name: String,

    pub size: u64,
    pub date_last_modified: String,

    pub files: Vec<FileDropFile>,
}

impl FileDropDir {
    pub fn new(name: String, dlm: SystemTime, path_to_dir: PathBuf) -> FileDropDir {
        let dt: DateTime<Local> = dlm.clone().into();

        return FileDropDir {
            name: name,
            size: self::FileDropDir::get_total_size(&path_to_dir),
            date_last_modified: format!("{}", dt.format("%v - %T")),
            files: self::FileDropDir::get_file_list(&path_to_dir),
        };
    }

    pub fn get_total_size(path: &PathBuf) -> u64 {
        let mut temp_count: u64 = 0;

        let entries = read_dir(path);
        if entries.is_ok() {
            for f in entries.unwrap() {
                if f.is_ok() {
                    let file = f.unwrap();
                    temp_count += file.metadata().unwrap().len();
                }
            }
        }

        return temp_count;
    }

    pub fn get_file_list(path: &PathBuf) -> Vec<FileDropFile> {
        let mut temp_list: Vec<FileDropFile> = Vec::new();

        let entries = read_dir(path);

        if entries.is_ok() {
            for f in entries.unwrap() {
                if f.is_ok() {
                    let file = f.unwrap();
                    temp_list.push(FileDropFile::new(
                        file.file_name()
                            .to_str()
                            .ok_or("/invalid filename/")
                            .unwrap()
                            .to_string(),
                        mime_guess::from_path(file.path())
                            .first_or_octet_stream()
                            .to_string(),
                        file.metadata().unwrap().len(),
                        file.metadata().unwrap().modified().unwrap(),
                    ));
                }
            }
        }

        return temp_list;
    }
}
