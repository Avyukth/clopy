use crate::domain::clipboard_item::{ClipboardContentType, ClipboardItem};
use crate::domain::clipboard_repository::{ClipboardRepository, RepositoryError};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::result::Result;

#[derive(Debug)]
pub struct FileClipboardRepository {
    base_dir: PathBuf,
}

impl FileClipboardRepository {
    pub fn new(base_dir: PathBuf) -> Self {
        Self { base_dir }
    }

    fn get_file_path(&self, id: u32) -> PathBuf {
        self.base_dir.join(format!("{}.json", id))
    }
}

impl ClipboardRepository for FileClipboardRepository {
    fn save_item(&self, item: ClipboardItem) -> Result<(), RepositoryError> {
        let file_path = self.get_file_path(item.id());
        let content = serde_json::to_string(&item).map_err(RepositoryError::Serde)?;
        fs::write(file_path, content).map_err(RepositoryError::Io)
    }

    fn get_item(&self, id: u32) -> Result<Option<ClipboardItem>, RepositoryError> {
        let file_path = self.get_file_path(id);
        if !file_path.exists() {
            return Ok(None);
        }
        let content = fs::read_to_string(file_path).map_err(RepositoryError::Io)?;
        let item = serde_json::from_str(&content).map_err(RepositoryError::Serde)?;
        Ok(Some(item))
    }

    fn get_all_items(&self) -> Result<Vec<ClipboardItem>, RepositoryError> {
        let entries = fs::read_dir(&self.base_dir).map_err(RepositoryError::Io)?;
        let mut items = Vec::new();
        for entry in entries {
            let entry = entry.map_err(RepositoryError::Io)?;
            let content = fs::read_to_string(entry.path()).map_err(RepositoryError::Io)?;
            let item = serde_json::from_str(&content).map_err(RepositoryError::Serde)?;
            items.push(item);
        }
        Ok(items)
    }
}
