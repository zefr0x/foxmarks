use std::fs::copy;
use std::path::PathBuf;
use std::string::String;

use configparser::ini::Ini;
use tempfile::NamedTempFile;

/// Returns a PathBuf for the selected profile places database file.
/// It will search for a default profile for the selected type or you can use a custom one.
///
/// # Arguments
/// * `firefox_type` - A u8 that refer to a firefox type.
///  `firefox`: 0
///  `firefox-esr`: 1
///  `firefox-dev`: 2
/// * `custom_profile_id` - Optional to use by passing a String with a profile is.
///  like: `xxxxxxxx.banking-profile`
///  A list could be found in `~/.mozilla/firefox/profiles.ini`
fn get_database_location(firefox_type: u8, custom_profile_id: Option<String>) -> PathBuf {
    let firefox_home_dir: PathBuf = match dirs::home_dir() {
        Some(path) => path,
        None => panic!("Can't find home directory."),
    }
    .join(".mozilla/firefox");

    let mut profiles = Ini::new();
    profiles
        .load(firefox_home_dir.join("profiles.ini"))
        .unwrap();

    let mut profile_id: Option<String> = None;

    match custom_profile_id {
        Some(custom_profile_id) => profile_id = Some(custom_profile_id),
        None => {
            for section in profiles.sections() {
                match profiles.get(&section, "Default") {
                    Some(id) => {
                        if (id.ends_with("default-release") && firefox_type == 0)
                            || (id.ends_with("default-esr") && firefox_type == 1)
                            || (id.ends_with("dev-edition-default") && firefox_type == 2)
                        {
                            profile_id = Some(id);
                        }
                    }
                    None => continue,
                }
            }
        }
    }

    match profile_id {
        Some(profile_id) => return firefox_home_dir.join(profile_id).join("places.sqlite"),
        None => panic!("Can't find a profile id."),
    }
}

/// Returns a PathBuf for a tempfile.
/// Since the database is locked when firefox is running, we need to copy it to a tmpfile to use it.
///
/// # Arguments
/// * `database_location` - The database file's path.
fn get_temp_database(database_location: PathBuf) -> NamedTempFile {
    let temp_database_file = NamedTempFile::new().unwrap();

    // Copy the whole database file to a temp file.
    copy(database_location.as_path(), temp_database_file.path()).unwrap();

    return temp_database_file;
}

/// Returns sqlite connection and the temp_database.
fn get_database_connection(
    firefox_type: u8,
    custom_profile_id: Option<String>,
) -> (sqlite::Connection, NamedTempFile) {
    let database_location = get_database_location(firefox_type, custom_profile_id);

    let temp_database = get_temp_database(database_location);

    // Return temp_database, so it will be in the scope and the temp file will not be deleted.
    // We need it also to close it at the end properly.
    return (sqlite::open(temp_database.path()).unwrap(), temp_database);
}

pub fn fetch_bookmarks(
    firefox_type: u8,
    custom_profile_id: Option<String>,
    column_delimiter: &str,
    row_delimiter: &str,
) {
    let (database_connection, temp_database) =
        get_database_connection(firefox_type, custom_profile_id);

    let mut cursor = database_connection
        .prepare(
            "SELECT A.title, B.url FROM moz_bookmarks AS A JOIN moz_places AS B ON(A.fk = B.id);",
        )
        .unwrap()
        .into_cursor();

    loop {
        let row: sqlite::Row = match cursor.next() {
            Some(thing) => match thing {
                Ok(row) => row,
                Err(_) => continue,
            },
            None => break,
        };
        print!(
            "{}{column_delimiter}{}{row_delimiter}",
            match row.try_get::<String, _>(0) {
                Ok(name) => name,
                Err(_) => continue,
            },
            match row.try_get::<String, _>(1) {
                Ok(name) => name,
                Err(_) => continue,
            }
        );
    }

    temp_database.close().unwrap();
}

pub fn fetch_history(
    firefox_type: u8,
    custom_profile_id: Option<String>,
    column_delimiter: &str,
    row_delimiter: &str,
) {
    let (database_connection, temp_database) =
        get_database_connection(firefox_type, custom_profile_id);

    todo!();

    temp_database.close().unwrap();
}
