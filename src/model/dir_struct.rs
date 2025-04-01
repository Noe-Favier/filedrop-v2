extern crate serde;
use super::file_struct::FileDropFile;
use std::{env, fs::read_dir, path::PathBuf, time::SystemTime};
use chrono::prelude::{DateTime, Local};
use std::fs::DirEntry;

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
        read_dir(path).unwrap()
            .filter_map(|entry| entry.ok())
            .flat_map(|entry| {
                if entry.path().is_file() {
                    vec![FileDropDir::create_file_drop_file(entry, sub)].into_iter()
                } else {
                    Self::get_file_list(&entry.path(), true).into_iter()
                }
            })
            .collect()
    }

    fn create_file_drop_file(file: DirEntry, sub: bool) -> FileDropFile {
        let filename = if sub {
            file.path()
                .to_str()
                .unwrap_or("/invalid sub file name/")
                .replace(&env::var("files_path").unwrap_or_else(|_| "./files".to_string()), ".")
        } else {
            file.file_name()
                .to_str()
                .unwrap_or("/invalid filename/")
                .to_string()
        };

        FileDropFile::new(
            filename,
            mime_guess::from_path(file.path()).first_or_octet_stream().to_string(),
            file.metadata().unwrap().len(),
            file.metadata().unwrap().modified().unwrap(),
        )
    }
}
