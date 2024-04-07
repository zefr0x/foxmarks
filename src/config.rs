use configparser::ini::Ini;
use std::path::PathBuf;

#[must_use]
pub fn load(config_path: Option<&PathBuf>) -> Ini {
    //! # Panics
    //! When config file doesn't exist or can't detect xdg config directory.
    let config_path = config_path.map_or_else(
        #[allow(clippy::unwrap_used)]
        || dirs::config_dir().unwrap().join("foxmarks/config"),
        std::borrow::ToOwned::to_owned,
    );

    let mut config = configparser::ini::Ini::new();

    if config_path.exists() {
        #[allow(clippy::unwrap_used)]
        config.load(config_path).unwrap();
    }

    config
}
