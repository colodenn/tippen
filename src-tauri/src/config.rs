use std::collections::HashMap;
use std::hash;

use crate::theme::Theme;
use crate::utils;
use serde_json::Value;
pub struct Config {
    pub theme: String,
    pub volume: u8,
    pub enabled: bool,
}

pub fn create_config() -> bool {
    println!("Creating config file...");
    let thocky_dir = utils::get_thocky_dir();
    let config_file = thocky_dir.join("config.json");

    if utils::check_if_file_exists(config_file.to_str().unwrap()) {
        println!("Config file already exists.");
        return false;
    }

    let config = r#"{
        "theme": "@codenn-thocky",
        "volume": 50,
        "enabled": true
    }"#;
    let _ = std::fs::write(config_file, config);
    return true;
}

pub fn update_config() -> bool {
    return true;
}

pub fn get_selected_theme() -> String {
    let thocky_dir = utils::get_thocky_dir();
    let config_file = thocky_dir.join("config.json");
    let config = std::fs::read_to_string(config_file).unwrap();
    let config: Value = serde_json::from_str(&config).unwrap();
    return format!("{}", config["theme"].as_str().unwrap());
}

pub fn read_config() -> Config {
    let thocky_dir = utils::get_thocky_dir();
    let config_file = thocky_dir.join("config.json");
    let config = std::fs::read_to_string(config_file).unwrap();
    let config: Value = serde_json::from_str(&config).unwrap();
    let theme = format!("{}", config["theme"].as_str().unwrap());
    let volume = format!("{}", config["volume"].as_u64().unwrap());
    let enabled = format!("{}", config["enabled"].as_bool().unwrap());
    return Config {
        theme: theme,
        volume: volume.parse::<u8>().unwrap(),
        enabled: enabled.parse::<bool>().unwrap(),
    };
}

// TODO THEME
pub fn read_config_theme(theme: String) -> Theme {
    let thocky_dir = utils::get_thocky_dir();
    let theme_dir = thocky_dir.join("themes").join(theme);
    let config_file = theme_dir.join("config.json");
    let config = std::fs::read_to_string(config_file).unwrap();
    let config: Value = serde_json::from_str(&config).unwrap();

    let id = format!("{}", config["id"].as_str().unwrap());
    let name = format!("{}", config["name"].as_str().unwrap());
    let sound = format!("{}", config["sound"].as_str().unwrap());
    let author = format!("{}", config["author"].as_str().unwrap());

    let mut hashmap = HashMap::new();

    let keys = config["keys"].as_object().unwrap().clone();
    // let keys = std::collections::HashMap::new();

    for (key, value) in keys {
        let key = format!("{}", key);
        let value = format!("{}", value.as_str().unwrap());
        hashmap.insert(key, value);
    }

    return Theme {
        id: id,
        name: name,
        author: author,
        sound: sound,
        keys: hashmap,
    };
}
