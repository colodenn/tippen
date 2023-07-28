// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{CustomMenuItem, SystemTrayMenuItem, SystemTraySubmenu};
use tauri::{Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};
mod keyboard_listener;
use std::thread;

#[derive(Clone, serde::Serialize)]
struct Payload {
    mode: String,
    message: String,
}

fn main() {
    // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
    let enable = CustomMenuItem::new("enable".to_string(), "Disable").accelerator("Cmd+E");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit").accelerator("Cmd+Q");
    let volume_0 = CustomMenuItem::new("0".to_string(), "0%");

    let volume_25 = CustomMenuItem::new("25".to_string(), "25%");
    let volume_50 = CustomMenuItem::new("50".to_string(), "50%");

    let volume_75 = CustomMenuItem::new("75".to_string(), "75%");
    let volume_100 = CustomMenuItem::new("100".to_string(), "100%").selected();

    let thock_1 = CustomMenuItem::new("thock_1", "Thocky Keys").selected();
    let thock_2 = CustomMenuItem::new("thock_2", "Cloudy Keys");

    let volume_menu = SystemTrayMenu::new()
        .add_item(volume_0)
        .add_item(volume_25)
        .add_item(volume_50)
        .add_item(volume_75)
        .add_item(volume_100);
    let volume = SystemTraySubmenu::new("Volume", volume_menu);

    let switches_menu = SystemTrayMenu::new().add_item(thock_1).add_item(thock_2);
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
                _ => {}
            },
            _ => {}
        })
        .setup(move |app| {
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
