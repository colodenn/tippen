use crate::utils;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Theme {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "author")]
    pub author: String,

    #[serde(rename = "sound")]
    pub sound: String,

    #[serde(rename = "keys")]
    pub keys: HashMap<String, String>,
}

pub fn load_theme(theme_name: String) {
    println!("Loading theme: {}", theme_name);
    let thocky_dir = utils::get_thocky_dir();
}

pub fn get_themes() -> Vec<String> {
    println!("Getting themes...");
    let mut all_themes: Vec<String> = Vec::new();
    let thocky_dir = utils::get_thocky_dir();
    let themes_dir = thocky_dir.join("themes");
    let themes = std::fs::read_dir(themes_dir).unwrap();
    for theme in themes {
        let theme = theme.unwrap();
        let theme_name = theme.file_name();
        println!("Theme: {:?}", theme_name);
        let _ = all_themes.push(theme_name.to_str().unwrap().to_string());
    }
    return all_themes;
}
