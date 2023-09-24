// Declare the submodules within the `infrastructure` module
pub mod config;
pub mod file_clipboard_repository;

// Re-export items to make them available to the parent module or as public API
pub use config::Config;
pub use file_clipboard_repository::FileClipboardRepository;
