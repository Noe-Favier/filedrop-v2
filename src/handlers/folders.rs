use poem::error::InternalServerError;
use poem::handler;
use crate::model::folder_dto::FolderDto;
use crate::model::so::app_config::AppConfig;

#[handler]
pub async fn get_folders(env: AppConfig) -> Result<Vec<FolderDto>> {
    let mut result = Vec::new();
    let entries = env.file_path.read_dir().map_err(|_| InternalServerError())?;

    for entry in entries {
        let entry = entry.map_err(|_| InternalServerError("Invalid entry"))?;
        let path = entry.path();
        if path.is_dir() {
            match FileDto::new_from_path(path) {
                Ok(dto) => result.push(dto),
                Err(_) => continue,
            }
        }
    }

    Ok(result)
}