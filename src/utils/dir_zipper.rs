use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use walkdir::WalkDir;
use zip::{write::FileOptions, CompressionMethod::Deflated, ZipWriter};

/// Zips the contents of a directory into a destination file
///
/// # Arguments
/// * `path_to_dir` - The directory to zip
/// * `dest` - The destination file to write the zip to
///
/// # Returns
/// * `Result<(), ZipError>` - Result indicating success or failure
pub fn zip_dir(path_to_dir: impl AsRef<Path>, dest: &File) -> Result<(), Box<dyn std::error::Error>> {
    let path_to_dir = path_to_dir.as_ref();
    let mut zip = ZipWriter::new(dest.try_clone()?);

    // Buffer used for reading files
    let mut buffer = vec![0; 8192]; // Use a fixed size buffer instead of growing vector

    for entry in WalkDir::new(path_to_dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();

        // Create relative path from the directory being zipped
        let name = match path.strip_prefix(path_to_dir) {
            Ok(name) => name,
            Err(e) => {
                eprintln!("Path stripping error: {:?} for {:?}", e, path);
                continue;
            }
        };

        // Skip empty names (root directory)
        if name.as_os_str().is_empty() {
            continue;
        }

        if path.is_file() {
            // Create appropriate file options based on file metadata
            let options = create_file_options(path)?;

            // Add file to zip
            zip.start_file(name.to_string_lossy(), options)?;

            // Stream the file instead of loading it entirely into memory
            let mut file = File::open(path)?;
            loop {
                let read_bytes = file.read(&mut buffer)?;
                if read_bytes == 0 {
                    break;
                }
                zip.write_all(&buffer[..read_bytes])?;
            }
        } else if path.is_dir() {
            // Add directory to zip
            zip.add_directory(name.to_string_lossy(), FileOptions::default())?;
        }
    }

    zip.finish()?;
    Ok(())
}

/// Creates appropriate FileOptions based on file metadata
fn create_file_options(path: &Path) -> Result<FileOptions<()>, Box<dyn std::error::Error>> {
    let mut options = FileOptions::default()
        .compression_method(Deflated);

    #[cfg(unix)]
    {
        let metadata = std::fs::metadata(path)?;
        use std::os::unix::fs::PermissionsExt;
        let permissions = metadata.permissions().mode();
        options = options.unix_permissions(permissions);
    }
    #[cfg(not(unix))]
    {
        // Use sensible defaults for non-unix platforms
        let is_executable = path.extension().map_or(false, |ext|
            ext == "exe" || ext == "bat" || ext == "cmd" || ext == "sh",
        );

        options = options.unix_permissions(if is_executable { 0o755 } else { 0o644 });
    }

    Ok(options)
}