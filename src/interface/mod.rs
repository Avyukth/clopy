// Declare the submodules within the `interface` module
pub mod clipboard_controller;
pub mod error_handler;
pub mod routes;

pub use clipboard_controller::ClipboardController;
pub use error_handler::handle_error;
pub use routes::configure_routes;
