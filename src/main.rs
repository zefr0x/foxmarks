use core::str::FromStr;
use std::path::PathBuf;

use foxmarks::cli;
use foxmarks::config;
use foxmarks::database::DataBase;

fn main() {
    let matches = cli::build().get_matches();

    // Load config file.
    let config_object = config::load(matches.get_one::<PathBuf>("config-path"));

    // Set some general options.
    let firefox_type = matches
        .get_one::<cli::FirefoxType>("firefox-type")
        .map_or_else(
            || {
                cli::FirefoxType::from_str(
                    &config_object
                        .get("database", "firefox_type")
                        .unwrap_or_else(|| "Release".to_owned()),
                )
                .expect("Non-valid firefox type specified")
            },
            std::borrow::ToOwned::to_owned,
        );

    let firefox_home_path = matches.get_one::<String>("firefox-home-path").map_or_else(
        || config_object.get("database", "firefox_home_path"),
        |id| Some(id.to_owned()),
    );

    let profile_path = matches.get_one::<String>("profile-path").map_or_else(
        || config_object.get("database", "profile_path"),
        |id| Some(id.to_owned()),
    );

    let column_delimiter = matches.get_one::<String>("column-delimiter").map_or_else(
        || {
            config_object
                .get("output", "column_delimiter")
                .unwrap_or_else(|| String::from(";"))
        },
        std::borrow::ToOwned::to_owned,
    );

    let row_delimiter = matches.get_one::<String>("row-delimiter").map_or_else(
        || {
            config_object
                .get("output", "row_delimiter")
                .unwrap_or_else(|| String::from("\n"))
        },
        std::borrow::ToOwned::to_owned,
    );

    // TODO: Find better and more general way to do that.
    let column_delimiter = column_delimiter
        .replace("\\t", "\t")
        .replace("\\n", "\n")
        .replace("\\r", "\r");
    let row_delimiter = row_delimiter
        .replace("\\t", "\t")
        .replace("\\n", "\n")
        .replace("\\r", "\r");

    let db = DataBase::new(&firefox_type, firefox_home_path, profile_path).connect();

    // Match subcommad, and set sub option if available.
    match matches.subcommand() {
        Some(("bookmarks", _)) => {
            db.fetch_bookmarks(&column_delimiter, &row_delimiter);
        }
        Some(("history", _)) => {
            db.fetch_history(&column_delimiter, &row_delimiter);
        }
        _ => unreachable!(),
    }

    db.close();
}
