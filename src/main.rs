use std::path::PathBuf;
use std::string::String;

use clap::builder::NonEmptyStringValueParser;
use clap::{Arg, ArgAction, Command};

mod config;
mod database;

fn cli_command() -> Command<'static> {
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
        .arg(
            Arg::new("firefox-type")
                .short('t')
                .long("firefox-type")
                .action(ArgAction::Set)
                .value_name("type")
                .value_parser(clap::value_parser!(u8).range(0..3))
                .help("0 for firefox-release, 1 for firefox-esr, 2 for firefox-dev"),
        )
        .arg(
            Arg::new("profile-id")
                .short('p')
                .long("profile-id")
                .action(ArgAction::Set)
                .value_name("id")
                .value_parser(NonEmptyStringValueParser::new())
                .help("A custom profile id to be used rather then the defualt ones")
                .conflicts_with("firefox-type"),
        )
        .arg(
            Arg::new("column-delimiter")
                .long("column-delimiter")
                .action(ArgAction::Set)
                .value_name("delimiter")
                .help("A delimiter to seprate the columns of the output"),
        )
        .arg(
            Arg::new("row-delimiter")
                .long("row-delimiter")
                .action(ArgAction::Set)
                .value_name("delimiter")
                .help("A delimiter to seprate the rows of the output"),
        )
        .arg(
            Arg::new("config-file")
                .long("config-file")
                .action(ArgAction::Set)
                .value_name("PATH")
                .value_parser(clap::value_parser!(PathBuf))
                .help("Path to a costum config file"),
        )
        .subcommand(Command::new("bookmarks").about("Get browser bookmarks"))
        .subcommand(clap::Command::new("history").about("Get browsing history"))
}

fn main() {
}
