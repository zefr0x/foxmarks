use configparser::ini::Ini;
use std::path::PathBuf;

pub fn load(config_path: Option<&PathBuf>) -> Ini {
    let config_path = match config_path {
        Some(path) => path.to_owned(),
        None => dirs::config_dir().unwrap().join("foxmarks/config"),
    };

    let mut config = configparser::ini::Ini::new();

    if config_path.exists() {
        config.load(config_path).unwrap();
    }

    config
}
