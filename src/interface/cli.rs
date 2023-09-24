use crate::application::clipboard_service::ClipboardService;
use crate::application::dto::ClipboardContentDto;
use clap::{App, AppSettings, Arg, SubCommand};
use std::sync::Arc;

pub fn run_cli(service: Arc<ClipboardService>) {
    let matches = App::new("Clipboard Store")
        .version("1.0")
        .author("Author Name <author@example.com>")
        .about("Stores clipboard data")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("save")
                .about("Saves clipboard data")
                .arg(
                    Arg::with_name("TYPE")
                        .help("Type of the clipboard data (text, image, filepath, other)")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("CONTENT")
                        .help("The actual content of the clipboard data")
                        .required(true)
                        .index(2),
                ),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Gets clipboard data by id")
                .arg(
                    Arg::with_name("ID")
                        .help("The id of the clipboard data")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("save", Some(save_matches)) => {
            let content_type = save_matches.value_of("TYPE").unwrap();
            let content = save_matches.value_of("CONTENT").unwrap();
            // Handle the 'save' subcommand
            // You can call the appropriate service method here
            // For example:
            // service.save_clipboard_data(ClipboardContentDto::from(content_type, content));
        }
        ("get", Some(get_matches)) => {
            let id = get_matches.value_of("ID").unwrap();
            // Handle the 'get' subcommand
            // You can call the appropriate service method here
            // For example:
            // service.get_clipboard_data(id.parse().unwrap());
        }
        _ => unreachable!(),
    }
}
