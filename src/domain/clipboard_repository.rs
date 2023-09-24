use crate::domain::clipboard_item::{ClipboardContentType, ClipboardItem};
use std::result::Result;
use thiserror::Error;

// Define a custom error type for the repository operations
#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}

pub trait ClipboardRepository {
    fn save_item(&self, item: ClipboardItem) -> Result<(), RepositoryError>;
    fn get_item(&self, id: u32) -> Result<Option<ClipboardItem>, RepositoryError>;
    fn get_all_items(&self) -> Result<Vec<ClipboardItem>, RepositoryError>;
}
