use std::path::PathBuf;
use crate::model::enums::io_item_type::IoItemType;

pub trait IoItem {
    fn get_type(&self) -> IoItemType;
    fn get_name(&self) -> &str;
    fn get_path(&self) -> String;
    fn get_size(&self) -> &u64;
    fn get_last_modified(&self) -> &chrono::NaiveDate;
    fn new_from_path(path: &PathBuf) -> Self;
}