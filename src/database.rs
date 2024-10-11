use core::{marker::PhantomData, mem};
use std::path::PathBuf;

use tempfile::NamedTempFile;

use crate::cli::FirefoxType;

struct Row {
    title: String,
    url: String,
}

pub struct NotConnected;
pub struct Connected;

pub struct DataBase<State = NotConnected> {
    database_location: PathBuf,
    temp_database: Option<NamedTempFile>,
    connection: Option<rusqlite::Connection>,
    state: PhantomData<State>,
}

impl DataBase {
    #[must_use]
    #[expect(clippy::use_self, reason = "`Self` doesn't accept type parameters")]
    pub fn new(
        firefox_type: &FirefoxType,
        firefox_home_path: Option<String>,
        custom_profile_path: Option<String>,
    ) -> DataBase<NotConnected> {
        Self {
            database_location: Self::get_database_location(
                firefox_type,
                firefox_home_path,
                custom_profile_path,
            ),
            temp_database: None,
            connection: None,
            state: PhantomData::<NotConnected>,
        }
    }

    /// Returns a [`PathBuf`] for the selected profile places database file.
    /// It will search for a default profile for the selected type or you can use a custom one.
    ///
    /// # Arguments
    /// * `custom_profile_path` - Optional to be used by passing a String with a profile path.
    ///   like: `xxxxxxxx.banking-profile`
    ///   A list could be found in `~/.mozilla/firefox/profiles.ini`
    fn get_database_location(
        firefox_type: &FirefoxType,
        firefox_home_path: Option<String>,
        custom_profile_path: Option<String>,
    ) -> PathBuf {
        let firefox_home_dir = firefox_home_path.map_or_else(
            || {
                dirs::home_dir()
                    .map_or_else(|| panic!("Can't find home directory."), |path| path)
                    .join(".mozilla/firefox")
            },
            PathBuf::from,
        );

        let mut profiles = configparser::ini::Ini::new();
        #[expect(clippy::unwrap_used, reason = "Default panic message is perfect")]
        profiles
            .load(firefox_home_dir.join("profiles.ini"))
            // The panic message provided is great
            .unwrap();

        let mut profile_path: Option<String> = None;

        match custom_profile_path {
            Some(custom_profile_path) => profile_path = Some(custom_profile_path),
            None => {
                for section in profiles.sections() {
                    match profiles.get(&section, "Default") {
                        Some(id) => {
                            if (id.ends_with("default-release")
                                && firefox_type == &FirefoxType::Release)
                                || (id.ends_with("default-esr")
                                    && firefox_type == &FirefoxType::Esr)
                                || ((id.ends_with("dev-edition-default")
                                    || id.ends_with("Default_Dev"))
                                    && firefox_type == &FirefoxType::Dev)
                            {
                                profile_path = Some(id);
                            }
                        }
                        None => continue,
                    }
                }
            }
        }

        profile_path.map_or_else(
            || {
                panic!(
                    "Can not find any suitable default profile id for firefox type {firefox_type}"
                )
            },
            |profile_path| firefox_home_dir.join(profile_path).join("places.sqlite"),
        )
    }
}

/// Create connection to the temp database.
impl DataBase<NotConnected> {
    #[must_use]
    pub fn connect(&mut self) -> DataBase<Connected> {
        //! # Panics
        //! When can't connect to the database or cna't create temp file and copy the origin database.
        let temp_database = self.get_temp_database();
        #[expect(clippy::unwrap_used, reason = "Should panic if failed")]
        let connection = rusqlite::Connection::open(temp_database.path()).unwrap();

        DataBase {
            database_location: mem::take(&mut self.database_location),
            temp_database: Some(temp_database),
            connection: Some(connection),
            state: PhantomData::<Connected>,
        }
    }

    /// Since the database is locked when firefox is running, we need to copy it to a tmpfile to use it.
    fn get_temp_database(&self) -> NamedTempFile {
        #[expect(clippy::unwrap_used, reason = "Should panic if failed")]
        let temp_database_file = NamedTempFile::new().unwrap();

        // Copy the whole database file to a temp file.
        #[expect(clippy::unwrap_used, reason = "Should panic if failed")]
        std::fs::copy(self.database_location.as_path(), temp_database_file.path()).unwrap();

        temp_database_file
    }
}

impl DataBase<Connected> {
    pub fn close(mut self) {
        //! # Panics
        //! When can't close the database connection or the temp file.
        if let Some(connection) = self.connection {
            #[expect(clippy::unwrap_used, reason = "Should panic if failed")]
            connection.close().unwrap();
            self.connection = None;
        }
        if let Some(temp_database) = self.temp_database {
            #[expect(clippy::unwrap_used, reason = "Should panic if failed")]
            temp_database.close().unwrap();
            self.connection = None;
        }
    }

    // TODO: Make it a generator function
    pub fn fetch_bookmarks(&self, column_delimiter: &str, row_delimiter: &str) {
        //! # Panics
        //! When don't get the expected results from the database query.
        #[expect(clippy::unwrap_used, reason = "This call is correct")]
        let mut statement = self
            .connection
            .as_ref()
            .unwrap()
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

        #[expect(clippy::unwrap_used, reason = "Should panic if failed")]
        for row in row_iter.unwrap() {
            let row = row.unwrap();
            print!("{}{column_delimiter}{}{row_delimiter}", row.title, row.url);
        }
    }

    // TODO: Make it a generator function
    pub fn fetch_history(&self, column_delimiter: &str, row_delimiter: &str) {
        //! # Panics
        //! When don't get the expected results from the database query.
        #[expect(clippy::unwrap_used, reason = "This call is correct")]
        let mut statement = self
            .connection
            .as_ref()
            .unwrap()
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

        #[expect(clippy::unwrap_used, reason = "Should panic if failed")]
        for row in row_iter.unwrap() {
            let row = row.unwrap();
            print!("{}{column_delimiter}{}{row_delimiter}", row.title, row.url);
        }
    }
}
