use crate::model::interfaces::io_item::IoItem;

#[derive(Clone, serde::Serialize)]
pub struct FolderDto {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub date_last_modified: chrono::NaiveDate,

    pub content: Vec<Box<dyn IoItem>>,
}

impl IoItem for FolderDto {
    fn get_type(&self) -> crate::model::enums::io_item_type::IoItemType {
        crate::model::enums::io_item_type::IoItemType::Folder
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
}