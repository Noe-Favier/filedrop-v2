use std::fs::File;
use std::path::PathBuf;
use crate::model::interfaces::io_item::IoItem;

#[derive(Clone, serde::Serialize)]
pub struct FileDto {
    pub name: String,
    pub path: PathBuf,
    pub size: u64,
    pub mime_type: String,
    pub date_last_modified: chrono::NaiveDate,
}

impl IoItem for FileDto {
    fn get_type(&self) -> crate::model::enums::io_item_type::IoItemType {
        crate::model::enums::io_item_type::IoItemType::File
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_path(&self) -> String {
        self.path.clone()
    }

    fn get_size(&self) -> &u64 {
        &self.size
    }

    fn get_last_modified(&self) -> &chrono::NaiveDate {
        &self.date_last_modified
    }

    fn new_from_path(path: PathBuf) -> Result<FileDto, std::io::Error> {
        // io
        let f: File = File::open(&path)?;
        let metadata = f.metadata()?;
        // modified date
        let system_time = metadata.modified()?;
        let datetime: chrono::DateTime<chrono::Utc> = system_time.into();
        let date_last_modified = datetime.date_naive();
        // name
        let name = path.file_name()
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid filename"))?
            .to_string_lossy()
            .to_string();

        Ok(FileDto {
            path: path.clone(),
            size: metadata.len(),
            mime_type: mime_guess::from_path(&path).first_or_octet_stream().to_string(),
            name,
            date_last_modified,
        })
    }
}

