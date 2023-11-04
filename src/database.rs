use std::marker::PhantomData;
use std::path::PathBuf;
use std::rc::Rc;

use tempfile::NamedTempFile;

use crate::cli::FirefoxType;

struct Row {
    title: String,
    url: String,
}

pub struct NotConnected;
pub struct Connected;

pub struct DataBase<State = NotConnected> {
    database_location: Rc<PathBuf>,
    temp_database: Option<NamedTempFile>,
    connection: Option<rusqlite::Connection>,
    state: PhantomData<State>,
}

impl DataBase {
    pub fn new(
        firefox_type: FirefoxType,
        custom_profile_path: Option<String>,
    ) -> DataBase<NotConnected> {
        Self {
            database_location: Rc::new(Self::get_database_location(
                firefox_type,
                custom_profile_path,
            )),
            temp_database: None,
            connection: None,
            state: PhantomData::<NotConnected>,
        }
    }

    /// Returns a PathBuf for the selected profile places database file.
    /// It will search for a default profile for the selected type or you can use a custom one.
    ///
    /// # Arguments
    /// * `custom_profile_path` - Optional to be used by passing a String with a profile path.
    ///  like: `xxxxxxxx.banking-profile`
    ///  A list could be found in `~/.mozilla/firefox/profiles.ini`
    fn get_database_location(
        firefox_type: FirefoxType,
        custom_profile_path: Option<String>,
    ) -> PathBuf {
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
                            if (id.ends_with("default-release")
                                && firefox_type == FirefoxType::Release)
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
}

impl DataBase<NotConnected> {
    /// Create connection to the temp database.
    pub fn connect(&self) -> DataBase<Connected> {
        let temp_database = self.get_temp_database();
        let connection = rusqlite::Connection::open(temp_database.path()).unwrap();

        DataBase {
            database_location: self.database_location.clone(),
            temp_database: Some(temp_database),
            connection: Some(connection),
            state: PhantomData::<Connected>,
        }
    }

    /// Since the database is locked when firefox is running, we need to copy it to a tmpfile to use it.
    fn get_temp_database(&self) -> NamedTempFile {
        let temp_database_file = NamedTempFile::new().unwrap();

        // Copy the whole database file to a temp file.
        std::fs::copy(self.database_location.as_path(), temp_database_file.path()).unwrap();

        temp_database_file
    }
}

impl DataBase<Connected> {
    pub fn close(mut self) {
        if let Some(connection) = self.connection {
            connection.close().unwrap();
            self.connection = None;
        }
        if let Some(temp_database) = self.temp_database {
            temp_database.close().unwrap();
            self.connection = None;
        }
    }

    // TODO: Make it a generator function
    pub fn fetch_bookmarks(&self, column_delimiter: String, row_delimiter: String) {
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

        for row in row_iter.unwrap() {
            let row = row.unwrap();
            print!("{}{column_delimiter}{}{row_delimiter}", row.title, row.url)
        }
    }

    // TODO: Make it a generator function
    pub fn fetch_history(&self, column_delimiter: String, row_delimiter: String) {
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

        for row in row_iter.unwrap() {
            let row = row.unwrap();
            print!("{}{column_delimiter}{}{row_delimiter}", row.title, row.url)
        }
    }
}
