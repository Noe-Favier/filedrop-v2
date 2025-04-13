#[derive(Clone, serde::Serialize)]
pub struct FolderLightDto {
    pub name: String,
    pub size: u64,
    pub content_length: usize,
}