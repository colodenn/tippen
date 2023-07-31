use directories::UserDirs;
use std::fs;

pub fn create_dir_if_not_exists() {
    println!("Creating sound theme directory...");

    let dir = get_thocky_dir().join("themes");
    let _ = fs::create_dir_all(dir);
}

pub fn get_thocky_dir() -> std::path::PathBuf {
    if let Some(user_dirs) = UserDirs::new() {
        let dir = user_dirs.home_dir().join(".thocky");
        println!("User directories: {:?}", dir);
        return dir;
    } else {
        panic!("Couldn't find home directory.");
    };
}
