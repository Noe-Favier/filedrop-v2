use crate::model::enums::io_item_type::IoItemType;

#[derive(Clone, serde::Serialize)]
pub struct SerializableIoItem {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub date_last_modified: String,

    pub item_type: IoItemType,
    // optional
    pub mime_type: Option<String>,
    pub content: Option<Vec<SerializableIoItem>>,
}