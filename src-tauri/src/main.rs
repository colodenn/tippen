// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{CustomMenuItem, SystemTrayMenuItem, SystemTraySubmenu};
use tauri::{Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};
mod keyboard_listener;
use std::path::Path;
use std::thread;
mod config;
mod dir_watcher;
mod theme;
mod utils;
#[derive(Clone, serde::Serialize)]
struct Payload {
    mode: String,
    message: String,
}

fn main() {
    // let themes = Vec::<theme::Theme>::new();

    let enable = CustomMenuItem::new("enable".to_string(), "Disable").accelerator("Cmd+E");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit").accelerator("Cmd+Q");

    let volume_0 = CustomMenuItem::new("0".to_string(), "0%");
    let volume_25 = CustomMenuItem::new("25".to_string(), "25%");
    let volume_50 = CustomMenuItem::new("50".to_string(), "50%");
    let volume_75 = CustomMenuItem::new("75".to_string(), "75%");
    let volume_100 = CustomMenuItem::new("100".to_string(), "100%").selected();

    let volume_menu = SystemTrayMenu::new()
        .add_item(volume_0)
        .add_item(volume_25)
        .add_item(volume_50)
        .add_item(volume_75)
        .add_item(volume_100);
    let volume = SystemTraySubmenu::new("Volume", volume_menu);

    let mut switches_menu = SystemTrayMenu::new();

    let selected_theme = config::get_selected_theme();

    for theme in theme::get_themes() {
        if selected_theme == theme {
            let theme_item = CustomMenuItem::new(theme.clone(), theme.clone()).selected();
            switches_menu = switches_menu.add_item(theme_item);
        } else {
            let theme_item = CustomMenuItem::new(theme.clone(), theme.clone());
            switches_menu = switches_menu.add_item(theme_item);
        }
    }

    let switches = SystemTraySubmenu::new("Switches", switches_menu);

    let tray_menu = SystemTrayMenu::new()
        .add_item(enable)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_submenu(volume)
        .add_submenu(switches)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    let system_tray = SystemTray::new().with_menu(tray_menu);
    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|_app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                _ => {
                    println!("Clicked on menu item {}", id);
                }
            },
            _ => {}
        })
        .setup(move |app| {
            print!("Setting up...");
            thread::spawn(move || {
                let file_change =
                    dir_watcher::watch(Path::new("C:\\Users\\corne\\.thocky\\themes"));
                match file_change {
                    Ok(change) => println!("Watching...: {:?}", change),
                    Err(err) => println!("Error: {:?}", err),
                }
            });

            utils::create_dir_if_not_exists();
            theme::get_themes();
            config::create_config();
            let wv = app.get_window("main").unwrap();

            thread::spawn(move || {
                keyboard_listener::run_listener(move |s: &str, s1: &str| {
                    if let Err(err) = wv.emit(
                        "keypress",
                        Payload {
                            mode: String::from(s),
                            message: String::from(s1),
                        },
                    ) {
                        eprintln!("Error while emitting event: {:?}", err);
                    }
                })
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
