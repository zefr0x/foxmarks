use std::path::PathBuf;
use std::string::String;

use clap::builder::NonEmptyStringValueParser;
use clap::{Arg, ArgAction, Command};

mod config;
mod database;

fn cli() -> Command<'static> {
    Command::new(env!("CARGO_PKG_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        // FIXME:
        // .about(format!(
        //     "{}\nHomePage: {}\nLicense: {}",
        //     env!("CARGO_PKG_DESCRIPTION"),
        //     env!("CARGO_PKG_HOMEPAGE"),
        //     env!("CARGO_PKG_LICENSE")
        // ))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .disable_help_subcommand(true)
        .arg(
            Arg::new("firefox-type")
                .short('t')
                .long("firefox-type")
                .action(ArgAction::Set)
                .value_name("type")
                .value_parser(clap::value_parser!(u8).range(0..3))
                .help("0 for firefox-release, 1 for firefox-esr, 2 for firefox-dev")
                .display_order(0),
        )
        .arg(
            Arg::new("profile-id")
                .short('p')
                .long("profile-id")
                .action(ArgAction::Set)
                .value_name("id")
                .value_parser(NonEmptyStringValueParser::new())
                .help("A custom profile id to be used rather then the defualt ones")
                .long_help("A custom profile id to be used rather then the defualt ones.\nYou can find a list of the profiles in `~/.mozilla/firefox/profiles.ini` file, or just list the directories in `~/.mozilla/firefox` and there names are the profiles IDs.\nBy default it will detect the default profile for every firefox-type.")
                .conflicts_with("firefox-type")
                .display_order(1),
        )
        .arg(
            Arg::new("column-delimiter")
                .long("column-delimiter")
                .action(ArgAction::Set)
                .value_name("delimiter")
                .help("A delimiter to seprate the columns of the output")
                .display_order(2),
        )
        .arg(
            Arg::new("row-delimiter")
                .long("row-delimiter")
                .action(ArgAction::Set)
                .value_name("delimiter")
                .help("A delimiter to seprate the rows of the output")
                .display_order(3),
        )
        .arg(
            Arg::new("config-path")
                .long("config")
                .action(ArgAction::Set)
                .value_name("FILE")
                .value_parser(clap::value_parser!(PathBuf))
                .help("Path to a costum config file")
                .display_order(4),
        )
        .subcommand(
            Command::new("bookmarks")
                .about("Get browser bookmarks")
                .display_order(0),
        )
        .subcommand(
            clap::Command::new("history")
                .about("Get browsing history")
                .display_order(1),
        )
}

fn main() {
    let matches = cli().get_matches();

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
        // FIXME: The escape character is being escaped, so \n will be \\n.
        None => config_object
            .get("output", "row_delimiter")
            .unwrap_or(String::from("\n")),
    };

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
