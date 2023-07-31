use crate::utils;
pub struct Theme {
    pub name: String,
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
