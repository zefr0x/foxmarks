use core::{fmt::Display, str::FromStr};
use std::path::PathBuf;

#[derive(PartialEq, Eq, Clone)]
pub enum FirefoxType {
    Release,
    Esr,
    Dev,
}

impl FromStr for FirefoxType {
    type Err = String;

    fn from_str(source: &str) -> Result<Self, String> {
        match source {
            "Release" => Ok(Self::Release),
            "Esr" => Ok(Self::Esr),
            "Dev" => Ok(Self::Dev),
            _ => Err(source.to_owned()),
        }
    }
}

impl Display for FirefoxType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Release => "Release".to_owned(),
                Self::Esr => "Esr".to_owned(),
                Self::Dev => "Dev".to_owned(),
            }
        )
    }
}

impl clap::ValueEnum for FirefoxType {
    fn value_variants<'src>() -> &'src [Self] {
        &[Self::Release, Self::Esr, Self::Dev]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(clap::builder::PossibleValue::new(self.to_string()))
    }
}

#[must_use]
pub fn build() -> clap::Command {
    use clap::builder::NonEmptyStringValueParser;
    use clap::{Arg, ArgAction, Command};

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
                .value_parser(clap::builder::EnumValueParser::<FirefoxType>::new())
                .help("To find default created profile for a specific firefox version")
                .display_order(0),
        )
        .arg(
            Arg::new("firefox-home-path")
                .short('f')
                .long("firefox-home-path")
                .action(ArgAction::Set)
                .value_name("path")
                .value_parser(NonEmptyStringValueParser::new())
                .help("A custom firefox home path to be used rather than `~/.mozilla/firefox`")
                .long_help("A custom firefox home path to be used rather than `~/.mozilla/firefox`.\nThis is usefull for flatpak users.")
                .display_order(1),
        )
        .arg(
            Arg::new("profile-path")
                .short('p')
                .long("profile-path")
                .action(ArgAction::Set)
                .value_name("id")
                .value_parser(NonEmptyStringValueParser::new())
                .help("A custom profile path to be used rather then the defualt ones")
                .long_help("A custom profile path to be used rather then the defualt ones.\nYou can find a list of the profiles by looking in ProfileX entries in `~/.mozilla/firefox/profiles.ini` file.\nBy default it will detect the default profile for every firefox-type, except if you are using a custom profile as your default one.")
                .conflicts_with("firefox-type")
                .display_order(2),
        )
        .arg(
            Arg::new("column-delimiter")
                .long("column-delimiter")
                .action(ArgAction::Set)
                .value_name("delimiter")
                .help("A delimiter to seprate the columns of the output")
                .display_order(3),
        )
        .arg(
            Arg::new("row-delimiter")
                .long("row-delimiter")
                .action(ArgAction::Set)
                .value_name("delimiter")
                .help("A delimiter to seprate the rows of the output")
                .display_order(4),
        )
        .arg(
            Arg::new("config-path")
                .long("config")
                .action(ArgAction::Set)
                .value_name("FILE")
                .value_parser(clap::value_parser!(PathBuf))
                .help("Path to a costum config file")
                .display_order(5),
        )
        .subcommand(
            Command::new("bookmarks")
                .about("Get browser bookmarks")
                .display_order(0),
        )
        .subcommand(
            Command::new("history")
                .about("Get browsing history")
                .display_order(1),
        )
}
