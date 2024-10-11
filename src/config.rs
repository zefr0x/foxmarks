use configparser::ini::Ini;
use std::path::PathBuf;

#[must_use]
pub fn load(config_path: Option<&PathBuf>) -> Ini {
    //! # Panics
    //! When config file doesn't exist or can't detect xdg config directory.
    let config_path = config_path.map_or_else(
        || {
            dirs::config_dir()
                .expect("Can't find user's config directory path")
                .join("foxmarks/config")
        },
        ToOwned::to_owned,
    );

    let mut config = Ini::new();

    if config_path.exists() {
        #[expect(clippy::unwrap_used, reason = "Default panic message is perfect")]
        config.load(config_path).unwrap();
    }

    config
}
