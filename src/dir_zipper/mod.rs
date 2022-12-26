use std::fs::read_dir;
use std::path::PathBuf;
use std::time::SystemTime;
use std::u8;
use std::{path, fs::File};
use rocket::futures::sink::Buffer;
use zip::write::FileOptions;
use zip::CompressionMethod;

use crate::dir_struct::FileDropDir;

#[path = "../dir_struct/mod.rs"] mod dir_struct;

pub fn get_zipped_vec(path_to_dir: String, path_to_parent: String) -> Vec<u8> { 
    //get file
    let parent = read_dir(path_to_parent).unwrap();
    let dir = parent.find(|entry| entry.expect("is not a valid ref").path().to_str().unwrap() == (path_to_dir.as_str())).expect("file not found").expect("failed extracting DirEntry");

    let filename: String = dir.file_name().to_str().expect("can't find requested dir").to_string();
    let date_lm: SystemTime = dir.metadata().unwrap().modified().unwrap();

    let dir_to_zip: FileDropDir = FileDropDir::new(
        filename.to_owned(),
        date_lm.to_owned(),
        dir.path()
    );

    let buf: Vec<u8> = Vec::new();

    let buffer: Buffer<u8>
    let mut buf: [u8; &dir_to_zip.size as usize] = [];

    let mut w = std::io::Cursor::new(buf);
    let mut zip = zip::ZipWriter::new(w);

    //create a buffer 
    //size of the buffer based on total size of dir
    //zip into buffer
    //return buffer
    //buffer will be sent to client as raw msg pack 

    /*
    let mut buffer: &'a tempfile::file::NamedTempFile;

    let mut zip = zip::ZipWriter::new(buffer);
    let options = FileOptions::default()
    .compression_method(CompressionMethod::Deflated)
    .unix_permissions(0o755);
    
    zip.start_file("hello_world.txt", options)?;
    zip.write(b"Hello, World!")?;
     */
    return buf;
}
