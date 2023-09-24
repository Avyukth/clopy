use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// Enum representing the different types of clipboard items
#[derive(Debug, Serialize, Deserialize)]
pub enum ClipboardContentType {
    Text(String),
    Image(PathBuf),
    FilePath(PathBuf),
    Other(PathBuf),
}

// Struct representing a clipboard item
#[derive(Debug, Serialize, Deserialize)]
pub struct ClipboardItem {
    id: u32,                                   // Unique identifier for the clipboard item
    content: ClipboardContentType,             // The actual content of the clipboard item
    created_at: chrono::DateTime<chrono::Utc>, // Timestamp representing when the clipboard item was created
}

impl ClipboardItem {
    // Constructor function to create a new ClipboardItem
    pub fn new(id: u32, content: ClipboardContentType) -> Self {
        Self {
            id,
            content,
            created_at: chrono::Utc::now(),
        }
    }

    // Getter methods to access the fields of ClipboardItem
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn content(&self) -> &ClipboardContentType {
        &self.content
    }

    pub fn created_at(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.created_at
    }
}
