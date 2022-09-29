use std::path::PathBuf;

use clap::builder::NonEmptyStringValueParser;
use clap::{Arg, ArgAction, Command};

pub fn build_cli() -> Command {
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
