/*
see :

https://github.com/zip-rs/zip/blob/f6357c59936b51c52146f35c6cf3c15dd206251d/examples/write_dir.rs
https://docs.rs/tempfile/latest/tempfile/struct.Builder.html
*/
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use zip::{write::FileOptions, CompressionMethod::Deflated, ZipWriter};

///zip the dir and return the path to the archive
pub fn zip_dir(path_to_dir: PathBuf, dest: &File) -> Result<bool, Box<dyn std::error::Error>> {
    // By closing the `TempDir` explicitly, we can check that it has
    // been deleted successfully. If we don't close it explicitly,
    // the directory will still be deleted when `dir` goes out
    // of scope, but we won't know whether deleting the directory
    // succeeded.
    //eg. drop(file);
    //    dir.close();

    //**************** ZIP THE DIR ****************//
    let mut zip = ZipWriter::new(dest.try_clone()?);

    let options = FileOptions::default()
        .compression_method(Deflated)
        .unix_permissions(0o755);

    let walkdir = WalkDir::new(&path_to_dir);
    let it = walkdir.into_iter();
    let mut buffer = Vec::new();

    for entry in it {
        let dir_entry = entry.unwrap();
        let path = dir_entry.path();
        let name = path
            .strip_prefix(Path::new(path_to_dir.to_str().unwrap()))
            .unwrap();

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            #[allow(deprecated)]
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            #[allow(deprecated)]
            zip.add_directory_from_path(name, options)?;
        }
    }

    zip.finish()?;
    //**************** RETURN THE FILE ****************//
    return Ok(true);
}
