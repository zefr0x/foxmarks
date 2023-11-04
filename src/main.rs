use std::path::PathBuf;
use std::str::FromStr;

use foxmarks::cli;
use foxmarks::config;
use foxmarks::database::DataBase;

fn main() {
    let matches = cli::build_cli().get_matches();

    // Load config file.
    let config_object = config::load(matches.get_one::<PathBuf>("config-path"));

    // Set some general options.
    let firefox_type = match matches.get_one::<cli::FirefoxType>("firefox-type") {
        Some(firefox_type) => firefox_type.to_owned(),
        None => cli::FirefoxType::from_str(
            &config_object
                .get("database", "firefox_type")
                .unwrap_or("Release".to_owned()),
        )
        .expect("Non-valid firefox type specified"),
    };

    let profile_path = match matches.get_one::<String>("profile-path") {
        Some(id) => Some(id.to_owned()),
        None => config_object.get("database", "profile_path"),
    };

    let column_delimiter = match matches.get_one::<String>("column-delimiter") {
        Some(delimiter) => delimiter.to_owned(),
        None => config_object
            .get("output", "column_delimiter")
            .unwrap_or_else(|| String::from(";")),
    };

    let row_delimiter = match matches.get_one::<String>("row-delimiter") {
        Some(delimiter) => delimiter.to_owned(),
        None => config_object
            .get("output", "row_delimiter")
            .unwrap_or_else(|| String::from("\n")),
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

    let db = DataBase::new(firefox_type, profile_path).connect();

    // Match subcommad, and set sub option if available.
    match matches.subcommand() {
        Some(("bookmarks", _)) => {
            db.fetch_bookmarks(column_delimiter, row_delimiter);
        }
        Some(("history", _)) => {
            db.fetch_history(column_delimiter, row_delimiter);
        }
        _ => unreachable!(),
    }

    db.close();
}
