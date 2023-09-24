use crate::domain::clipboard_item::{ClipboardContentType, ClipboardItem};
use crate::domain::clipboard_repository::ClipboardRepository;
use std::result::Result;
use std::sync::Arc;

// Define a custom error type for the service operations
#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] crate::domain::clipboard_repository::RepositoryError),
    // Add other error variants as needed
}

// Define a struct representing the ClipboardService
pub struct ClipboardService {
    repository: Arc<dyn ClipboardRepository>,
}

impl ClipboardService {
    // Constructor function to create a new ClipboardService
    pub fn new(repository: Arc<dyn ClipboardRepository>) -> Self {
        Self { repository }
    }

    // Method to save a clipboard item
    pub fn save_clipboard_data(
        &self,
        content: ClipboardContentType,
    ) -> Result<ClipboardItem, ServiceError> {
        let item = ClipboardItem::new(0, content); // ID is 0 as a placeholder, consider using a unique ID generator
        self.repository.save_item(item.clone())?;
        Ok(item)
    }

    // Method to get a clipboard item by id
    pub fn get_clipboard_data(&self, id: u32) -> Result<Option<ClipboardItem>, ServiceError> {
        self.repository.get_item(id)
    }

    // Method to get all clipboard items
    pub fn get_all_clipboard_data(&self) -> Result<Vec<ClipboardItem>, ServiceError> {
        self.repository.get_all_items()
    }
}
