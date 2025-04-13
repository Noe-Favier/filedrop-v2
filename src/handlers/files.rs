use crate::model::file_dto::FileDto;
use crate::model::interfaces::io_item::IoItem;
use crate::model::so::app_config::AppConfig;
use poem::{
    error::InternalServerError, handler, http::{header, StatusCode}, web::Path, IntoResponse, Response, Result,
};
use std::path::{Component, PathBuf};
use tokio::{fs::File, io::AsyncReadExt};

#[handler]
pub async fn get_files(env: AppConfig) -> Result<Vec<FileDto>> {
    let mut result = Vec::new();
    let entries = env.file_path.read_dir().map_err(|_| InternalServerError("Failed to read dir"))?;

    for entry in entries {
        let entry = entry.map_err(|_| InternalServerError("Invalid entry"))?;
        let path = entry.path();
        if path.is_file() {
            match FileDto::new_from_path(path) {
                Ok(dto) => result.push(dto),
                Err(_) => continue, // tu peux aussi logger
            }
        }
    }

    Ok(result)
}

#[handler]
pub async fn get_file(path: Path<String>, env: AppConfig) -> Result<FileDto> {
    let path = safe_path(&env.file_path, &path)?;
    Ok(FileDto::new_from_path(path).map_err(|_| InternalServerError("File not found"))?)
}

#[handler]
pub async fn download_file(path: Path<String>, env: AppConfig) -> Result<Response> {
    let path = safe_path(&env.file_path, &path)?;
    let mut file = File::open(&path).await.map_err(|_| InternalServerError("Can't open file"))?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await.map_err(|_| InternalServerError("Can't read file"))?;

    let filename = path.file_name().unwrap_or_default().to_string_lossy();
    let mime = mime_guess::from_path(&path).first_or_octet_stream();

    Ok(Response::builder()
        .header(header::CONTENT_TYPE, mime.as_ref())
        .header(header::CONTENT_DISPOSITION, format!("attachment; filename=\"{}\"", filename))
        .body(buffer)
        .into_response())
}

// Pour empêcher les attaques type "../../etc/passwd"
fn safe_path(base: &PathBuf, rel: &str) -> Result<PathBuf> {
    let rel_path = PathBuf::from(rel);
    if rel_path.components().any(|c| matches!(c, Component::ParentDir)) {
        return Err(poem::Error::from(StatusCode::BAD_REQUEST));
    }
    Ok(base.join(rel_path))
}
