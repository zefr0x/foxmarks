use configparser::ini::Ini;
use std::path::PathBuf;

pub fn load(config_file: Option<PathBuf>) -> Ini {
    let config_file = match config_file {
        Some(config_file) => config_file,
        None => dirs::config_dir().unwrap().join("foxmarks/config"),
    };

    let mut config = configparser::ini::Ini::new();

    if config_file.exists() {
        config.load(config_file).unwrap();
    }

    return config;
}
