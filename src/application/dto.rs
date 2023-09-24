use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// Enum representing the different types of clipboard content for DTO
#[derive(Debug, Serialize, Deserialize)]
pub enum ClipboardContentDto {
    Text(String),
    Image(PathBuf),
    FilePath(PathBuf),
    Other(PathBuf),
}

// Struct representing a clipboard item for DTO
#[derive(Debug, Serialize, Deserialize)]
pub struct ClipboardItemDto {
    pub id: u32,                      // Unique identifier for the clipboard item
    pub content: ClipboardContentDto, // The actual content of the clipboard item
    pub created_at: String,           // Timestamp representing when the clipboard item was created
}

impl ClipboardItemDto {
    // Constructor function to create a new ClipboardItemDto
    pub fn new(id: u32, content: ClipboardContentDto, created_at: String) -> Self {
        Self {
            id,
            content,
            created_at,
        }
    }
}
