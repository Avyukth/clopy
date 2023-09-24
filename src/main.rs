// mod app;

// use app::*;
// use leptos::*;

// fn main() {
//     mount_to_body(|cx| {
//         view! { cx,
//             <App/>
//         }
//     })
// }

mod application;
mod domain;
mod infrastructure;
mod interface;
mod utils;

use crate::application::clipboard_service::ClipboardService;
use crate::infrastructure::file_clipboard_repository::FileClipboardRepository;

fn main() {
    let clipboard_repository = FileClipboardRepository::new();
    let clipboard_service = ClipboardService::new(clipboard_repository);

    println!("Running non-Tauri related tasks...");
}
