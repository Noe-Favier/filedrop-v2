#[derive(Clone, serde::Serialize)]
pub struct FileLightDto {
    pub name: String,
    pub size: u64,
    pub mime_type: String,
}