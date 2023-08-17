use std::path::PathBuf;

use tempfile::NamedTempFile;

use crate::cli::FirefoxType;

struct Row {
    title: String,
    url: String,
}

/// Returns a PathBuf for the selected profile places database file.
/// It will search for a default profile for the selected type or you can use a custom one.
///
/// # Arguments
/// * `firefox_type` - A u8 that refer to a firefox type.
///  `firefox`: 0
///  `firefox-esr`: 1
///  `firefox-dev`: 2
/// * `custom_profile_path` - Optional to use by passing a String with a profile is.
///  like: `xxxxxxxx.banking-profile`
///  A list could be found in `~/.mozilla/firefox/profiles.ini`
fn get_database_location(firefox_type: FirefoxType, custom_profile_path: Option<String>) -> PathBuf {
    let firefox_home_dir: PathBuf = match dirs::home_dir() {
        Some(path) => path,
        None => panic!("Can't find home directory."),
    }
    .join(".mozilla/firefox");

    let mut profiles = configparser::ini::Ini::new();
    profiles
        .load(firefox_home_dir.join("profiles.ini"))
        .unwrap();

    let mut profile_path: Option<String> = None;

    match custom_profile_path {
        Some(custom_profile_path) => profile_path = Some(custom_profile_path),
        None => {
            for section in profiles.sections() {
                match profiles.get(&section, "Default") {
                    Some(id) => {
                        if (id.ends_with("default-release") && firefox_type == FirefoxType::Release)
                            || (id.ends_with("default-esr") && firefox_type == FirefoxType::Esr)
                            || ((id.ends_with("dev-edition-default")
                                || id.ends_with("Default_Dev"))
                                && firefox_type == FirefoxType::Dev)
                        {
                            profile_path = Some(id);
                        }
                    }
                    None => continue,
                }
            }
        }
    }

    match profile_path {
        Some(profile_path) => firefox_home_dir.join(profile_path).join("places.sqlite"),
        None => {
            panic!(
                "Can not find any suitable default profile id for firefox type {}",
                firefox_type.to_string()
            )
        }
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
    std::fs::copy(database_location.as_path(), temp_database_file.path()).unwrap();

    temp_database_file
}

/// Returns sqlite connection and the temp_database.
fn get_database_connection(
    firefox_type: FirefoxType,
    custom_profile_path: Option<String>,
) -> (rusqlite::Connection, NamedTempFile) {
    let database_location = get_database_location(firefox_type, custom_profile_path);

    let temp_database = get_temp_database(database_location);

    // Return temp_database, so it will be in the scope and the temp file will not be deleted.
    // We need it also to close it at the end properly.
    return (
        rusqlite::Connection::open(temp_database.path()).unwrap(),
        temp_database,
    );
}

// TODO: Make it a generator function
pub fn fetch_bookmarks(
    firefox_type: FirefoxType,
    custom_profile_path: Option<String>,
    column_delimiter: String,
    row_delimiter: String,
) {
    let (database_connection, temp_database) =
        get_database_connection(firefox_type, custom_profile_path);

    let mut statement = database_connection
        .prepare(
            "SELECT A.title, B.url
                FROM moz_bookmarks AS A JOIN moz_places AS B ON(A.fk = B.id)
                    ORDER BY B.visit_count DESC, A.lastModified DESC;",
        )
        .unwrap();

    let row_iter = statement.query_map([], |row| {
        Ok(Row {
            // Return the value if exists, if not, a default String (empty string).
            title: row.get(0).unwrap_or_default(),
            url: row.get(1).unwrap_or_default(),
        })
    });

    for row in row_iter.unwrap() {
        let row = row.unwrap();
        print!("{}{column_delimiter}{}{row_delimiter}", row.title, row.url)
    }

    temp_database.close().unwrap();
}

// TODO: Make it a generator function
pub fn fetch_history(
    firefox_type: FirefoxType,
    custom_profile_path: Option<String>,
    column_delimiter: String,
    row_delimiter: String,
) {
    let (database_connection, temp_database) =
        get_database_connection(firefox_type, custom_profile_path);

    let mut statement = database_connection
        .prepare(
            "SELECT B.title, B.url
                FROM moz_historyvisits AS A JOIN moz_places AS B ON(A.place_id = B.id)
                    ORDER BY A.visit_date DESC, B.visit_count DESC;",
        )
        .unwrap();

    let row_iter = statement.query_map([], |row| {
        Ok(Row {
            // Return the value if exists, if not, a default String (empty string).
            title: row.get(0).unwrap_or_default(),
            url: row.get(1).unwrap_or_default(),
        })
    });

    for row in row_iter.unwrap() {
        let row = row.unwrap();
        print!("{}{column_delimiter}{}{row_delimiter}", row.title, row.url)
    }

    temp_database.close().unwrap();
}
