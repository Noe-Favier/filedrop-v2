use chrono::prelude::{DateTime, Local};
use std::{env, fs::read_dir, path::PathBuf, time::SystemTime};

use super::file_struct::FileDropFile;

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
            files: self::FileDropDir::get_file_list(&path_to_dir, false),
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

    pub fn get_file_list(path: &PathBuf, sub: bool) -> Vec<FileDropFile> {
        //sub is used to mark file with full path
        let mut temp_list: Vec<FileDropFile> = Vec::new();

        let entries = read_dir(path);

        if entries.is_ok() {
            for f in entries.unwrap() {
                if f.is_err() {
                    continue;
                }
                let f = f.unwrap();
                if f.path().is_file() {
                    let file = f;

                    let filename: String;
                    if sub {
                        filename = file
                            .path()
                            .to_str()
                            .unwrap_or("/invalid sub file name/")
                            .to_string()
                            .replace(
                                &env::var("files_path").unwrap_or(String::from("./files")),
                                ".",
                            );
                    } else {
                        filename = file
                            .file_name()
                            .to_str()
                            .ok_or("/invalid filename/")
                            .unwrap()
                            .to_string();
                    }

                    temp_list.push(FileDropFile::new(
                        filename,
                        mime_guess::from_path(file.path())
                            .first_or_octet_stream()
                            .to_string(),
                        file.metadata().unwrap().len(),
                        file.metadata().unwrap().modified().unwrap(),
                    ));
                } else {
                    temp_list.append(&mut Self::get_file_list(&f.path(), true));
                }
            }
        }

        return temp_list;
    }
}
