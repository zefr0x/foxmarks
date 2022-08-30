use std::path::PathBuf;
use std::string::String;

mod cli;
mod config;
mod database;

fn main() {
    let matches = cli::build_cli().get_matches();

    // Load config file.
    let config_object = config::load(matches.get_one::<PathBuf>("config-path"));

    // Set some general options.
    let firefox_type = match matches.get_one::<u8>("firefox-type") {
        Some(type_num) => type_num.to_owned(),
        None => config_object
            .getint("database", "firefox_type")
            .unwrap()
            .unwrap_or(0) as u8,
    };

    let profile_id = match matches.get_one::<String>("profile-id") {
        Some(id) => Some(id.to_owned()),
        None => config_object.get("database", "profile_id"),
    };

    let column_delimiter = match matches.get_one::<String>("column-delimiter") {
        Some(delimiter) => delimiter.to_owned(),
        None => config_object
            .get("output", "column_delimiter")
            .unwrap_or(String::from(";")),
    };

    let row_delimiter = match matches.get_one::<String>("row-delimiter") {
        Some(delimiter) => delimiter.to_owned(),
        None => config_object
            .get("output", "row_delimiter")
            .unwrap_or(String::from("\n")),
    };

    // TODO: Find better and more general way to do that.
    let column_delimiter = column_delimiter
        .replace("\\t", "\t")
        .replace("\\n", "\n")
        .replace("\\r", "\r");
    let row_delimiter = row_delimiter
        .replace("\\t", "\t")
        .replace("\\n", "\n")
        .replace("\\r", "\r");

    // Match subcommad, and set sub option if available.
    match matches.subcommand() {
        Some(("bookmarks", _sub_matches)) => {
            database::fetch_bookmarks(firefox_type, profile_id, column_delimiter, row_delimiter);
        }
        Some(("history", _sub_matches)) => {
            database::fetch_history(firefox_type, profile_id, column_delimiter, row_delimiter);
        }
        _ => unreachable!(),
    }
}
